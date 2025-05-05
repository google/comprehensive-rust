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

use anyhow::Context;
use log::trace;
use mdbook::BookItem;
use mdbook::book::Book;
use mdbook::renderer::RenderContext;
use mdbook_exerciser::process;
use std::fs::{create_dir, remove_dir_all};
use std::io::stdin;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let context = RenderContext::from_json(&mut stdin()).context("Parsing stdin")?;

    let config = context
        .config
        .get_renderer("exerciser")
        .context("Missing output.exerciser configuration")?;

    let output_directory = Path::new(
        config
            .get("output-directory")
            .context(
                "Missing output.exerciser.output-directory configuration value",
            )?
            .as_str()
            .context("Expected a string for output.exerciser.output-directory")?,
    );

    let _ = remove_dir_all(output_directory);
    create_dir(output_directory).with_context(|| {
        format!("Failed to create output directory {:?}", output_directory)
    })?;

    process_all(&context.book, output_directory)?;

    Ok(())
}

fn process_all(book: &Book, output_directory: &Path) -> anyhow::Result<()> {
    for item in book.iter() {
        if let BookItem::Chapter(chapter) = item {
            trace!("Chapter {:?} / {:?}", chapter.path, chapter.source_path);
            if let Some(chapter_path) = &chapter.path {
                // Put the exercises in a subdirectory named after the chapter file,
                // without its parent directories.
                let chapter_output_directory =
                    output_directory.join(chapter_path.file_stem().with_context(
                        || format!("Chapter {:?} has no file stem", chapter_path),
                    )?);
                process(&chapter_output_directory, &chapter.content)?;
            }
        }
    }

    Ok(())
}
