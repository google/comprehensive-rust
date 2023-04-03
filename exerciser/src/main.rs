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

use anyhow::Context;
use exerciser::process;
use std::{
    env::args,
    fs::{create_dir, read_to_string},
    path::Path,
    process::exit,
};

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let args = args().collect::<Vec<_>>();

    if args.len() != 3 {
        eprintln!("Usage:");
        eprintln!(
            "  {} <src/exercises/exercise.md> <output directory>",
            args[0]
        );
        exit(1);
    }

    let input_filename = Path::new(&args[1]);
    let output_directory = Path::new(&args[2]);

    create_dir(output_directory).with_context(|| {
        format!("Failed to create output directory {:?}", output_directory)
    })?;

    let input_directory = input_filename.parent().with_context(|| {
        format!("Input file {:?} has no parent directory.", input_filename)
    })?;
    let input_contents = read_to_string(input_filename)
        .with_context(|| format!("Failed to open {:?}", input_filename))?;

    process(input_directory, output_directory, &input_contents)?;

    Ok(())
}
