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
use matter::matter;
use mdbook::book::{Book, BookItem};
use mdbook::preprocess::PreprocessorContext;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Frontmatter {
    minutes: Option<u64>,
}

pub fn remove_frontmatter(
    ctx: &PreprocessorContext,
    book: &mut Book,
) -> anyhow::Result<()> {
    let is_html = ctx.renderer == "html";
    book.for_each_mut(|chapter| {
        let BookItem::Chapter(chapter) = chapter else {
            return;
        };
        if let Some((frontmatter, mut content)) = matter(&chapter.content) {
            if !is_html {
                // For non-HTML renderers, just strip the frontmatter.
                chapter.content = content;
                return;
            }

            let frontmatter: Frontmatter = serde_yaml::from_str(&frontmatter)
                .with_context(|| {
                    format!("error parsing frontmatter in {:?}", chapter.source_path)
                })
                .unwrap();

            if let Some(minutes) = frontmatter.minutes {
                // Include the minutes in the speaker notes.
                let plural = if minutes == 1 { "minute" } else { "minutes" };
                content = content.replace("<details>", 
                    &format!("<details>\nThis slide should take about {minutes:?} {plural}. "));
            }
            chapter.content = content;
        }
    });
    Ok(())
}
