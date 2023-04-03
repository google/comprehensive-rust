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
use pulldown_cmark::{CowStr, Event, Parser, Tag};
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::Path,
};

const INCLUDE_START: &str = "{{#include ";
const INCLUDE_END: &str = "}}";
const FILENAME_START: &str = "<!-- File ";
const FILENAME_END: &str = " -->";

pub fn process(
    input_directory: &Path,
    output_directory: &Path,
    input_contents: &str,
) -> anyhow::Result<()> {
    let parser = Parser::new(input_contents);

    // Find a specially-formatted comment followed by a code block, and then call `write_output`
    // with the contents of the code block, to write to a file named by the comment. Code blocks
    // without matching comments will be ignored, as will comments which are not followed by a code
    // block.
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
                    write_output(text, input_directory, output_file)?;
                }
            }
            Event::End(Tag::CodeBlock(x)) => {
                info!("End   {:?}", x);
                current_file = None;
            }
            _ => {}
        }
    }

    Ok(())
}

/// Writes the given output file based on the given code text from the Markdown input, processing
/// include directives as necessary.
fn write_output(
    text: CowStr,
    input_directory: &Path,
    output_file: &mut File,
) -> anyhow::Result<()> {
    for line in text.lines() {
        info!("Line: {:?}", line);
        if let (Some(start), Some(end)) =
            (line.find(INCLUDE_START), line.find(INCLUDE_END))
        {
            let include = line[start + INCLUDE_START.len()..end].trim();
            info!("Include {:?}", include);
            if let Some(colon) = include.find(":") {
                write_include(
                    &include[0..colon],
                    Some(&include[colon + 1..]),
                    input_directory,
                    output_file,
                )?;
            } else {
                write_include(include, None, input_directory, output_file)?;
            }
        } else {
            output_file.write(line.as_bytes())?;
            output_file.write(b"\n")?;
        }
    }

    Ok(())
}

/// Writes the given `section` (or all, if it is `None`) of the given included file (relative to the
/// `input_directory`) to the `output_file`.
fn write_include(
    include_filename: &str,
    section: Option<&str>,
    input_directory: &Path,
    output_file: &mut File,
) -> anyhow::Result<()> {
    let full_include_filename = input_directory.join(include_filename);
    let input_file = read_to_string(full_include_filename)?;
    if let Some(section) = section {
        let start_anchor = format!("ANCHOR: {}", section);
        let end_anchor = format!("ANCHOR_END: {}", section);
        for line in input_file
            .lines()
            .skip_while(|line| !line.contains(&start_anchor))
            .skip(1)
            .take_while(|line| !line.contains(&end_anchor))
        {
            output_file.write(line.as_bytes())?;
            output_file.write(b"\n")?;
        }
    } else {
        output_file.write(input_file.as_bytes())?;
    }

    Ok(())
}
