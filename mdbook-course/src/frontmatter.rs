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
use mdbook::book::Chapter;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Frontmatter {
    pub minutes: Option<u64>,
    pub target_minutes: Option<u64>,
    pub course: Option<String>,
    pub session: Option<String>,
}

/// Split a chapter's contents into frontmatter and the remaining contents.
pub fn split_frontmatter(
    chapter: &Chapter,
) -> anyhow::Result<(Frontmatter, String)> {
    if let Some((frontmatter, content)) = matter(&chapter.content) {
        let frontmatter: Frontmatter = serde_yaml::from_str(&frontmatter)
            .with_context(|| {
                format!("error parsing frontmatter in {:?}", chapter.source_path)
            })?;

        Ok((frontmatter, content))
    } else {
        Ok((Frontmatter::default(), chapter.content.clone()))
    }
}
