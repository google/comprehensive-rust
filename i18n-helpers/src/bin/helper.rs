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
use std::io::{stdin, Read};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process;

/// A span in the translated document
struct TranslatedSpan {
    /// The range of characters in the source captured by this span.
    src_range: Range<usize>,

    /// The translation of this span of text.
    message: Message,
}

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

/// Extract the text from the catalog.
fn text_from_catalog(catalog: Catalog) -> (String, CatalogMetadata, Vec<TranslatedSpan>) {
    let mut document = String::new();
    let mut spans = vec![];

    for msg in catalog.messages {
        // ignore untranslated content
        if msg.get_msgstr().unwrap().is_empty()
            || msg.get_msgstr().unwrap() == msg.get_msgid().unwrap()
        {
            continue;
        }
        let start = document.len();
        document.push_str(&msg.get_msgid().unwrap());
        let end = document.len();
        document.push_str("\n\n");

        let src_range = cleanup_span(&document, Range { start, end });
        if src_range.end <= src_range.start {
            continue;
        }

        spans.push(TranslatedSpan {
            src_range,
            message: msg,
        });
    }

    (document, catalog.metadata, spans)
}

/// Re-parse the source using the new parser
fn new_parse(document: &str) -> Vec<Range<usize>> {
    let mut spans = vec![];
    for msg in extract_msgs(document) {
        spans.push(msg.span());
    }
    spans
}

fn merge(
    document: &str,
    translated_spans: &[TranslatedSpan],
    new_spans: &[Range<usize>],
) -> anyhow::Result<Vec<Message>> {
    let mut merged = vec![];
    let mut translated_spans = translated_spans.into_iter();
    for new_span in new_spans {
        // the translated spans comprising this new span
        let mut collected_spans = vec![];
        // cleanup the new span for purposes of matching
        let clean_span = cleanup_span(document, new_span.clone());
        while let Some(xlated_span) = translated_spans.next() {
            if xlated_span.src_range.end > clean_span.end {
                anyhow::bail!(
                    "span mismatch:\n---- new split:\n{:?}\n---- old splits:\n{:?}\n---- last message:\n{:#?}",
                    &document[clean_span.clone()],
                    &document[clean_span.start..xlated_span.src_range.end],
                    xlated_span.message,
                );
            }
            collected_spans.push(xlated_span);
            if xlated_span.src_range.end == clean_span.end {
                let first = &collected_spans[0].message;

                // draw msgid from the new span, so that it matches
                // new parses.
                let msgid = &document[new_span.clone()];

                // compose msgstr from the concatenation of all msgstr's in the collected
                // spans.
                let mut msgstr = String::from(first.get_msgstr().unwrap());
                for xlated_span in &collected_spans[1..] {
                    msgstr.push_str("\n\n");
                    msgstr.push_str(xlated_span.message.get_msgstr().unwrap());
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

    // for each translated span, construct a new message containing the concatentation of
    // the msgstr's from the source messages, and the messageid from the new span.
    Ok(merged)
}

fn process_po_file(filename: &str, output_filename: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(filename.to_string());
    let catalog = po_file::parse(&path).expect("Could not open .po file");

    let (document, metadata, translated_spans) = text_from_catalog(catalog);
    let new_spans = new_parse(&document);

    // write out the document content for debugging
    std::fs::write("document.md", &document).context("writing document")?;

    let merged =
        merge(&document, &translated_spans[..], &new_spans[..]).expect("merge failed");

    let mut catalog = Catalog::new();
    catalog.messages = merged;
    catalog.metadata = metadata;

    let output_path = PathBuf::from(output_filename.to_string());
    // po_file does not truncate before writing to a file, so do so on its behalf.
    if output_path.exists() {
        std::fs::remove_file(&output_path)
            .with_context(|| format!("Removing {}", output_path.display()))?;
    }
    po_file::write(&catalog, &output_path)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    process_po_file("po/ko.po", "po/ko-new.po")?;
    Ok(())
}
