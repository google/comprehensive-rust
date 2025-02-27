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

use log::{info, trace};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

const FILENAME_START: &str = "<!-- File ";
const FILENAME_END: &str = " -->";

pub fn process(output_directory: &Path, input_contents: &str) -> anyhow::Result<()> {
    let parser = Parser::new(input_contents);

    // Find a specially-formatted comment followed by a code block, and then call
    // `write_output` with the contents of the code block, to write to a file
    // named by the comment. Code blocks without matching comments will be
    // ignored, as will comments which are not followed by a code block.
    let mut next_filename: Option<String> = None;
    let mut current_file: Option<File> = None;
    for event in parser {
        trace!("{:?}", event);
        match event {
            Event::Html(html) => {
                let html = html.trim();
                if html.starts_with(FILENAME_START) && html.ends_with(FILENAME_END) {
                    next_filename = Some(
                        html[FILENAME_START.len()..html.len() - FILENAME_END.len()]
                            .to_string(),
                    );
                    info!("Next file: {:?}:", next_filename);
                }
            }
            Event::Start(Tag::CodeBlock(x)) => {
                info!("Start {:?}", x);
                if let Some(filename) = &next_filename {
                    let full_filename = output_directory.join(filename);
                    info!("Opening {:?}", full_filename);
                    if let Some(directory) = full_filename.parent() {
                        create_dir_all(directory)?;
                    }
                    current_file = Some(File::create(full_filename)?);
                    next_filename = None;
                }
            }
            Event::Text(text) => {
                info!("Text: {:?}", text);
                if let Some(output_file) = &mut current_file {
                    output_file.write_all(text.as_bytes())?;
                }
            }
            Event::End(TagEnd::CodeBlock) => {
                info!("End");
                current_file = None;
            }
            _ => {}
        }
    }

    Ok(())
}
