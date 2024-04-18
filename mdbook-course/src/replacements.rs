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

use crate::course::{Course, Courses, Segment, Session};
use mdbook::book::Chapter;
use regex::Regex;

lazy_static::lazy_static! {
    static ref DIRECTIVE: Regex = Regex::new(r#"\{\{%([^}]*)}}"#).unwrap();
}

/// Replace supported directives with the relevant content.
///
/// See the mdbook-course README for details.
#[allow(unused_variables)]
pub fn replace(
    courses: &Courses,
    course: Option<&Course>,
    session: Option<&Session>,
    segment: Option<&Segment>,
    chapter: &mut Chapter,
) {
    let Some(source_path) = &chapter.source_path else {
        return;
    };
    chapter.content = DIRECTIVE
        .replace(&chapter.content, |captures: &regex::Captures| {
            let directive_str = captures[1].trim();
            let directive: Vec<_> = directive_str.split_whitespace().collect();
            match directive.as_slice() {
                ["session", "outline"] if session.is_some() => {
                    session.unwrap().outline()
                }
                ["segment", "outline"] if segment.is_some() => {
                    segment.unwrap().outline()
                }
                ["course", "outline"] if course.is_some() => {
                    course.unwrap().schedule()
                }
                ["course", "outline", course_name] => {
                    let Some(course) = courses.find_course(course_name) else {
                        return captures[0].to_string();
                    };
                    course.schedule()
                }
                _ => directive_str.to_owned(),
            }
        })
        .to_string();
}
