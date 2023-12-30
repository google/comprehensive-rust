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

mod course;
mod frontmatter;
mod markdown;
mod replacements;
mod timing_info;

use crate::course::{Course, Courses};
use crate::markdown::duration;
use clap::{Arg, Command};
use mdbook::book::BookItem;
use mdbook::preprocess::CmdPreprocessor;
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

    if let Some(_) = matches.subcommand_matches("supports") {
        // Support all renderers.
        process::exit(0);
    }

    if let Err(e) = preprocess() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn timediff(actual: u64, target: u64) -> String {
    if actual > target {
        format!(
            "{}: {} OVER TARGET {}",
            duration(actual),
            duration(actual - target),
            duration(target)
        )
    } else if actual < target {
        format!(
            "{}: {} shorter than target {}",
            duration(actual),
            duration(target - actual),
            duration(target)
        )
    } else {
        format!("{}: right on time", duration(actual))
    }
}

fn print_summary(fundamentals: &Course) {
    eprintln!("Fundamentals: {}", timediff(fundamentals.minutes(), 8 * 3 * 60));

    eprintln!("Sessions:");
    for session in fundamentals {
        eprintln!("  {}: {}", session.name, timediff(session.minutes(), 3 * 60));
        for segment in session {
            eprintln!("    {}: {}", segment.name, duration(segment.minutes()));
        }
    }
}

fn preprocess() -> anyhow::Result<()> {
    let (ctx, book) = CmdPreprocessor::parse_input(stdin())?;
    let (courses, mut book) = Courses::extract_structure(book)?;
    let verbose = ctx
        .config
        .get_preprocessor("course")
        .and_then(|t| t.get("verbose"))
        .and_then(|v| v.as_bool())
        .unwrap_or_default();

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

    // Print a summary of times for the "Fundamentals" course.
    // Translations with a POT-Creation-Date before 2023-11-29 (when
    // we merged #1073) will have no frontmatter.
    if verbose {
        if let Some(fundamentals) = courses.find_course("Fundamentals") {
            print_summary(fundamentals);
        }
    }

    serde_json::to_writer(stdout(), &book)?;
    Ok(())
}
