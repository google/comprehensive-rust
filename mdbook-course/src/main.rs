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
use mdbook::preprocess::CmdPreprocessor;
use mdbook_course::frontmatter::remove_frontmatter;
use std::io::{stdin, stdout};
use std::process;

fn main() {
    pretty_env_logger::init();
    let app = Command::new("mdbook-course")
        .about("mdbook preprocessor for Comprehensive Rust")
        .subcommand(Command::new("supports").arg(Arg::new("renderer").required(true)));
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

fn preprocess() -> anyhow::Result<()> {
    let (ctx, mut book) = CmdPreprocessor::parse_input(stdin())?;

    remove_frontmatter(&ctx, &mut book)?;

    serde_json::to_writer(stdout(), &book)?;
    Ok(())
}
