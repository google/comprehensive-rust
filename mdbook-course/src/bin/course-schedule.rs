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

use clap::Command;
use mdbook::MDBook;
use mdbook_course::course::Courses;
use mdbook_course::markdown::duration;

fn main() {
    pretty_env_logger::init();
    let app = Command::new("mdbook-course")
        .about("mdbook preprocessor for Comprehensive Rust")
        .subcommand(Command::new("sessions").about("Show session summary (default)"))
        .subcommand(Command::new("segments").about("Show segment summary"))
        .subcommand(Command::new("pr").about("Show summary for a PR"));
    let matches = app.get_matches();

    let root_dir = ".";
    let mdbook = MDBook::load(root_dir).expect("Unable to load the book");
    let (courses, _) = Courses::extract_structure(mdbook.book)
        .expect("Unable to extract course structure");

    match matches.subcommand() {
        Some(("session", _)) | None => session_summary(&courses),
        Some(("pr", _)) => pr_summary(&courses),
        _ => unreachable!(),
    }
}

fn timediff(actual: u64, target: u64, slop: u64) -> String {
    if actual > target + slop {
        format!(
            "{} (\u{23f0} *{} too long*)",
            duration(actual),
            duration(actual - target),
        )
    } else if actual + slop < target {
        format!("{}: ({} short)", duration(actual), duration(target - actual),)
    } else {
        duration(actual).to_string()
    }
}

fn session_summary(courses: &Courses) {
    for course in courses {
        if course.target_minutes() == 0 {
            return;
        }
        for session in course {
            println!("### {} // {}", course.name, session.name);
            println!(
                "_{}_",
                timediff(session.minutes(), session.target_minutes(), 15)
            );
            println!();
            for segment in session {
                println!("* {} - _{}_", segment.name, duration(segment.minutes()));
            }
            println!();
        }
    }
}

fn pr_summary(courses: &Courses) {
    println!("## Course Schedule");
    println!("With this pull request applied, the course schedule is as follows:");
    for course in courses {
        if course.target_minutes() == 0 {
            return;
        }
        println!("### {}", course.name);
        println!("_{}_", timediff(course.minutes(), course.target_minutes(), 15));

        for session in course {
            println!(
                "* {} - _{}_",
                session.name,
                timediff(session.minutes(), session.target_minutes(), 5)
            );
        }
    }
}
