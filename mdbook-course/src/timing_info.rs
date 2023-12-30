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

use crate::course::Slide;
use mdbook::book::Chapter;

/// Insert timing information for this slide into the speaker notes.
pub fn insert_timing_info(slide: &Slide, chapter: &mut Chapter) {
    if slide.minutes > 0
        && !slide.is_sub_chapter(chapter)
        && chapter.content.contains("<details>")
    {
        // Include the minutes in the speaker notes.
        let minutes = slide.minutes;
        let plural = if slide.minutes == 1 { "minute" } else { "minutes" };
        let mut subslides = "";
        if slide.source_paths.len() > 1 {
            subslides = "and its sub-slides ";
        }
        let timing_message =
            format!("This slide {subslides}should take about {minutes} {plural}. ");
        chapter.content = chapter
            .content
            .replace("<details>", &format!("<details>\n{timing_message}"));
    }
}
