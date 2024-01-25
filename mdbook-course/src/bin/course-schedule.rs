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

use mdbook::MDBook;
use mdbook_course::course::{Course, Courses};
use mdbook_course::markdown::duration;

fn main() {
    pretty_env_logger::init();
    let root_dir = ".";
    let mdbook = MDBook::load(root_dir).expect("Unable to load the book");
    let (courses, _) = Courses::extract_structure(mdbook.book)
        .expect("Unable to extract course structure");

    println!("## Course Schedule");
    println!("With this pull request applied, the course schedule is as follows:");
    for course in &courses {
        print_summary(course);
    }
}

fn timediff(actual: u64, target: u64, slop: u64) -> String {
    if actual > target + slop {
        format!(
            "{} (\u{23f0} *{} too long*)",
            duration(actual),
            duration(actual - target),
        )
    } else if actual < target - slop {
        format!("{}: ({} short)", duration(actual), duration(target - actual),)
    } else {
        format!("{}", duration(actual))
    }
}

fn print_summary(course: &Course) {
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
