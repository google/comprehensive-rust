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

//! This binary allows us to execute tasks within the project by running
//! `cargo xtask <task>`. It can thus be used as a task automation tool.
//! For example instead of repeatedly running `cargo install` from the CLI
//! to install all the necessary tools for the project we can just run
//! `cargo xtask install-tools` and the logic defined here will install
//! the tools.

use anyhow::{Ok, Result, anyhow};
use clap::Parser;
use std::env;
use std::process::Command;

fn main() -> Result<()> {
    if let Err(e) = execute_task() {
        eprintln!("{e}");
        std::process::exit(-1);
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(
    about = "Binary for executing tasks within the Comprehensive Rust project"
)]
struct Args {
    #[arg(required = true, help = "The task to execute")]
    task: String,
}

fn execute_task() -> Result<()> {
    let task = Args::parse().task;
    match task.as_str() {
        "install-tools" => install_tools()?,
        _ => {
            return Err(anyhow!(unrecognized_task_string(task.as_str())));
        }
    }
    Ok(())
}

fn install_tools() -> Result<()> {
    println!("Installing project tools...");

    let install_args = vec![
        // The --locked flag is important for reproducible builds. It also
        // avoids breakage due to skews between mdbook and mdbook-svgbob.
        vec!["mdbook", "--locked", "--version", "0.4.48"],
        vec!["mdbook-svgbob", "--locked", "--version", "0.2.2"],
        vec!["mdbook-pandoc", "--locked", "--version", "0.10.1"],
        vec!["mdbook-i18n-helpers", "--locked", "--version", "0.3.6"],
        vec!["i18n-report", "--locked", "--version", "0.2.0"],
        // These packages are located in this repository
        vec!["--path", "mdbook-exerciser", "--locked"],
        vec!["--path", "mdbook-course", "--locked"],
    ];

    for args in &install_args {
        let status = Command::new(env!("CARGO"))
            .arg("install")
            .args(args)
            .status()
            .expect("Failed to execute cargo install");

        if !status.success() {
            let error_message = format!(
                "Command 'cargo install {}' exited with status code: {}",
                args.join(" "),
                status.code().unwrap()
            );
            return Err(anyhow!(error_message));
        }
    }

    Ok(())
}

fn unrecognized_task_string(task: &str) -> String {
    format!(
        "Unrecognized task '{task}'. Available tasks:

install-tools            Installs the tools the project depends on."
    )
}
