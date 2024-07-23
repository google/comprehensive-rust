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

use clap::{Arg, Command};
use mdbook::book::BookItem;
use mdbook::preprocess::CmdPreprocessor;
use mdbook_course::course::Courses;
use mdbook_course::{replacements, timing_info};
use std::io::{stdin, stdout};
use std::process;

fn main() {
    pretty_env_logger::init();
    let app = Command::new("mdbook-course")
        .about("mdbook preprocessor for Comprehensive Rust")
        .subcommand(
            Command::new("supports").arg(Arg::new("renderer").required(true)),
        );
    let matches = app.get_matches();

    if matches.subcommand_matches("supports").is_some() {
        // Support all renderers.
        process::exit(0);
    }

    if let Err(e) = preprocess() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn preprocess() -> anyhow::Result<()> {
    let (_, book) = CmdPreprocessor::parse_input(stdin())?;
    let (courses, mut book) = Courses::extract_structure(book)?;

    book.for_each_mut(|chapter| {
        if let BookItem::Chapter(chapter) = chapter {
            if let Some((course, session, segment, slide)) =
                courses.find_slide(chapter)
            {
                timing_info::insert_timing_info(slide, chapter);
                replacements::replace(
                    &courses,
                    Some(course),
                    Some(session),
                    Some(segment),
                    chapter,
                );
            } else {
                // Outside of a course, just perform replacements.
                replacements::replace(&courses, None, None, None, chapter);
            }
        }
    });

    serde_json::to_writer(stdout(), &book)?;
    Ok(())
}
