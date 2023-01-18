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

//! `xgettext` for `mdbook`
//!
//! This program works like `xgettext`, meaning it will extract
//! translatable strings from your book. The strings are saved in a
//! GNU Gettext `messages.pot` file in your build directory (typically
//! `po/messages.pot`).
//!
//! See `TRANSLATIONS.md` in the repository root for more information.

use anyhow::{anyhow, Context};
use mdbook::renderer::RenderContext;
use mdbook::BookItem;
use polib::catalog::Catalog;
use polib::message::Message;
use std::fs;
use std::io;

fn add_message(catalog: &mut Catalog, msgid: &str, source: &str) {
    let sources = match catalog.find_message(msgid) {
        Some(msg) => format!("{}\n{}", msg.source, source),
        None => String::from(source),
    };
    let message = Message::new_singular("", &sources, "", "", msgid, "");

    // Carefully update the existing message or add a new one. It's an
    // error to create a catalog with duplicate msgids.
    match catalog.find_message_index(msgid) {
        Some(&idx) => catalog.update_message_by_index(idx, message).unwrap(),
        None => catalog.add_message(message),
    }
}

fn create_catalog(ctx: &RenderContext) -> anyhow::Result<Catalog> {
    let mut catalog = Catalog::new();
    if let Some(title) = &ctx.config.book.title {
        catalog.metadata.project_id_version = String::from(title);
    }
    if let Some(lang) = &ctx.config.book.language {
        catalog.metadata.language = String::from(lang);
    }
    catalog.metadata.mime_version = String::from("1.0");
    catalog.metadata.content_type = String::from("text/plain; charset=UTF-8");
    catalog.metadata.content_transfer_encoding = String::from("8bit");

    let summary_path = ctx.config.book.src.join("SUMMARY.md");
    let summary = std::fs::read_to_string(ctx.root.join(&summary_path))?;

    // First, add all chapter names and part titles from SUMMARY.md.
    // The book items are in order of the summary, so we can assign
    // correct line numbers for duplicate lines by tracking the index
    // of our last search.
    let mut last_idx = 0;
    for item in ctx.book.iter() {
        let line = match item {
            BookItem::Chapter(chapter) => &chapter.name,
            BookItem::PartTitle(title) => title,
            BookItem::Separator => continue,
        };

        let idx = summary[last_idx..].find(line).ok_or_else(|| {
            anyhow!(
                "Could not find {line:?} in SUMMARY.md after line {} -- \
                 please remove any formatting from SUMMARY.md",
                summary[..last_idx].lines().count()
            )
        })?;
        last_idx += idx;
        let lineno = summary[..last_idx].lines().count();
        let source = format!("{}:{}", summary_path.display(), lineno);
        add_message(&mut catalog, line, &source);
    }

    // Next, we add the chapter contents.
    for item in ctx.book.iter() {
        if let BookItem::Chapter(chapter) = item {
            let path = match &chapter.path {
                Some(path) => ctx.config.book.src.join(path),
                None => continue,
            };
            for (lineno, paragraph) in i18n_helpers::extract_paragraphs(&chapter.content)
            {
                let source = format!("{}:{}", path.display(), lineno);
                add_message(&mut catalog, paragraph, &source);
            }
        }
    }

    Ok(catalog)
}

fn main() -> anyhow::Result<()> {
    let ctx = RenderContext::from_json(&mut io::stdin()).context("Parsing stdin")?;
    let cfg = ctx
        .config
        .get_renderer("xgettext")
        .ok_or_else(|| anyhow!("Could not read output.xgettext configuration"))?;
    let path = cfg
        .get("pot-file")
        .ok_or_else(|| anyhow!("Missing output.xgettext.pot-file config value"))?
        .as_str()
        .ok_or_else(|| anyhow!("Expected a string for output.xgettext.pot-file"))?;
    fs::create_dir_all(&ctx.destination)
        .with_context(|| format!("Could not create {}", ctx.destination.display()))?;
    let output_path = ctx.destination.join(path);
    if output_path.exists() {
        fs::remove_file(&output_path)
            .with_context(|| format!("Removing {}", output_path.display()))?
    }
    let catalog = create_catalog(&ctx).context("Extracting messages")?;
    polib::po_file::write(&catalog, &output_path)
        .with_context(|| format!("Writing messages to {}", output_path.display()))?;

    Ok(())
}
