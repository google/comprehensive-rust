// Copyright 2024 Google LLC
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
use mdbook_course::course::Courses;
use std::fs;
use std::path::Path;

fn main() {
    pretty_env_logger::init();
    let root_dir = ".";
    let mdbook = MDBook::load(root_dir).expect("Unable to load the book");
    let (courses, _) = Courses::extract_structure(mdbook.book)
        .expect("Unable to extract course structure");

    let src_dir = Path::new("src");
    for course in &courses {
        println!("# COURSE: {}", course.name);
        for session in course {
            println!("# SESSION: {}", session.name);
            for segment in session {
                println!("# SEGMENT: {}", segment.name);
                for slide in segment {
                    println!("# SLIDE: {}", slide.name);
                    for path in &slide.source_paths {
                        let content =
                            fs::read_to_string(src_dir.join(path)).unwrap();
                        println!("{}", content);
                    }
                }
            }
        }
    }
}
