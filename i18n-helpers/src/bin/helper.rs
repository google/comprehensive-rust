#![allow(unused_imports)]
// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Helper for issue 318, where we are changing how files are segmented into messages. This is
//! complicated by translation files that are out-of-date to the most recent source files.
//!
//! The idea here is to reconstruct the source from the translation file, keeping track of the old
//! message break locations. We then re-parse that source with the new parser, and then attempt to
//! align the old and new message boundaries.
//!
//! We reconstruct the entire source as one large markdown file, acknowledging that this is not
//! quite a valid markdown document. We omit any content that is not translated (has no msgstr).
//!
//! For each new message:
//!
//! - if the message's start and end boundaries align with start and end boundaries of old messages
//! (not necessarily the same message), then output a message that is the concatenation of the
//! spanned old messages.
//! - if the message's start or end boundaries do not align with any old boundaries, that is an
//! error, so output an appropriate error message and expect the user to update the input files.
//!
//! If there are no errors, the resulting catalog is output to stdout.

use anyhow::{anyhow, Context};
use i18n_helpers::extract_msgs;
use mdbook::book::Book;
use mdbook::preprocess::{CmdPreprocessor, PreprocessorContext};
use mdbook::BookItem;
use polib::catalog::Catalog;
use polib::message::Message;
use polib::metadata::CatalogMetadata;
use polib::po_file;
use semver::{Version, VersionReq};
use std::collections::HashMap;
use std::io::{stdin, Read};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process;

/// trim trailing newlines, space, and </details>, from the range (but leave them in the source)
fn cleanup_span(document: &str, mut span: Range<usize>) -> Range<usize> {
    while span.end > span.start {
        if document[..span.end].ends_with("\n") || document[..span.end].ends_with(" ") {
            span.end -= 1;
        } else if document[..span.end].ends_with("</details>") {
            span.end -= 10;
        } else {
            break;
        }
    }
    span
}

fn clone_message(msg: &Message) -> Message {
    Message::new_singular(
        &msg.comments,
        &msg.source,
        &msg.flags,
        &msg.msgctxt,
        msg.get_msgid().expect("expected singular message"),
        msg.get_msgstr().expect("expected singular message"),
    )
}

/// Extract the text of each source file from the catalog.
fn files_from_catalog(
    catalog: Catalog,
) -> (HashMap<String, Vec<Message>>, CatalogMetadata) {
    // filename -> line number -> message chunk
    let mut documents: HashMap<String, HashMap<usize, Message>> = HashMap::new();

    // Gather all messages, keyed by their source. The catalog deduplicates messages with
    // the same msgid, so re-duplicate those once for each source location.
    for msg in catalog.messages {
        for srcline in msg.source.lines() {
            for srcstr in srcline.split(' ') {
                let mut parts = srcstr.split(':');
                let filename = parts.next().expect("expected filename").to_string();
                let lineno = parts.next().expect("expected line number");
                let lineno = lineno
                    .parse()
                    .expect("expected line number to be an integer");
                documents
                    .entry(filename)
                    .or_default()
                    .insert(lineno, clone_message(&msg));
            }
        }
    }

    // Now, organize those into vectors of messages in order by filename
    let documents = documents
        .drain()
        .map(|(filename, mut messages)| {
            let mut messages: Vec<(usize, Message)> = messages.drain().collect();
            messages.sort_by_key(|(lineno, _)| *lineno);
            (
                filename,
                messages.drain(..).map(|(_, msg)| msg).collect(),
            )
        })
        .collect();
    (documents, catalog.metadata)
}

/// Re-parse the source using the new parser
fn new_parse(document: &str) -> Vec<Range<usize>> {
    let mut spans = vec![];
    for msg in extract_msgs(document) {
        spans.push(msg.span());
    }
    spans
}

fn merge(messages: &[Message]) -> anyhow::Result<Vec<Message>> {
    // A reconstruction of the un-translated source document.
    let mut untranslated = String::new();

    // A reconstruction of the translated document.
    let mut translated = String::new();

    // The character spans, in `untranslated`, of the old messages. Indices in this
    // vec match those in `messages`.
    let mut old_spans = vec![];

    for msg in messages {
        let start = untranslated.len();
        untranslated.push_str(msg.get_msgid().unwrap());
        let end = untranslated.len();
        untranslated.push_str("\n\n");

        translated.push_str(msg.get_msgstr().unwrap());
        translated.push_str("\n\n");

        old_spans.push(Range { start, end });
    }

    // Character spans, again in `untranslated`, using the new parser.
    let new_spans = new_parse(&untranslated);

    let mut merged = vec![];
    let mut old_spans = old_spans.into_iter().enumerate();
    for new_span in new_spans {
        // the index of the first old span in this new span
        let mut first_index = None;

        // cleanup the new span for purposes of matching
        let clean_span = cleanup_span(&untranslated, new_span.clone());
        while let Some((i, old_span)) = old_spans.next() {
            if first_index.is_none() {
                first_index = Some(i);
            }
            let old_span = cleanup_span(&untranslated, old_span);
            if old_span.end > clean_span.end {
                anyhow::bail!(
                    "span mismatch:\n---- new split:\n{:?}\n---- old splits:\n{:?}\n---- last message:\n{:#?}",
                    &untranslated[clean_span.clone()],
                    &untranslated[clean_span.start..old_span.end],
                    messages[i],
                );
            }
            let end = old_span.end;
            if end == clean_span.end {
                // the collection of messages comprising this new message.
                let src_messages = &messages[first_index.unwrap()..=i];
                // base the new message on the first of the old messages.
                let first = &messages[0];

                // draw msgid from the new span, so that it matches
                // new parses.
                let msgid = &untranslated[new_span.clone()];

                let mut msgstr = String::new();
                // If any of the source messages contain a translation, then include them all,
                // defaulting to the msgid for anything not containing a translation.
                // compose msgstr from the concatenation of all non-empty msgstr's in the collected
                // spans.
                if src_messages.iter().any(|msg| msg.get_msgstr().unwrap() != "") {
                    let msgstrs: Vec<_> = src_messages.iter().map(|msg| {
                        let mut msgstr = msg.get_msgstr().unwrap();
                        if msgstr == "" {
                            msgstr = msg.get_msgid().unwrap();
                        }
                        msgstr.as_ref()
                    }).collect();
                    msgstr = msgstrs.join("\n\n");
                }
                merged.push(Message::new_singular(
                    &first.comments,
                    &first.source,
                    &first.flags,
                    &first.msgctxt,
                    msgid,
                    &msgstr,
                ));

                // move on to the next span in new_spans
                break;
            }
        }
    }

    Ok(merged)
}

fn process_po_file(filename: &str, output_filename: &str) -> anyhow::Result<()> {
    println!("--{filename}");
    let path = PathBuf::from(filename.to_string());
    let catalog = po_file::parse(&path).expect("Could not open .po file");

    let (mut files, metadata) = files_from_catalog(catalog);
    let mut new_catalog = Catalog::new();
    new_catalog.metadata = metadata;

    let mut files: Vec<_> = files.drain().collect();
    files.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, messages) in files {
        for msg in merge(&messages)? {
            match new_catalog.find_message_index(msg.get_msgid().unwrap()) {
                Some(&idx) => new_catalog.update_message_by_index(idx, msg).unwrap(),
                None => new_catalog.add_message(msg),
            }
        }
    }

    let output_path = PathBuf::from(output_filename.to_string());
    // po_file does not truncate before writing to a file, so do so on its behalf.
    if output_path.exists() {
        std::fs::remove_file(&output_path)
            .with_context(|| format!("Removing {}", output_path.display()))?;
    }
    po_file::write(&new_catalog, &output_path)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process_po_file("po/ko.po", "po/ko-new.po")?;
    process_po_file("po/da.po", "po/da-new.po")?;
    process_po_file("po/de.po", "po/de-new.po")?;
    process_po_file("po/pt-BR.po", "po/pt-BR-new.po")?;
    Ok(())
}
