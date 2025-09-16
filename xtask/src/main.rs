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

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    execute_task()
}

#[derive(Parser)]
#[command(
    about = "Binary for executing tasks within the Comprehensive Rust project"
)]
struct Cli {
    /// The task to execute
    #[command(subcommand)]
    task: Task,
}

#[derive(Subcommand)]
enum Task {
    /// Installs the tools the project depends on.
    InstallTools,
    /// Runs the web driver tests in the tests directory.
    WebTests {
        /// Optional 'book html' directory - if set, will also refresh the list
        /// of slides used by slide size test.
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// Tests all included Rust snippets.
    RustTests,
    /// Starts a web server with the course.
    Serve {
        /// ISO 639 language code (e.g. da for the Danish translation).
        #[arg(short, long)]
        language: Option<String>,

        /// Directory to place the build. If not provided, defaults to the book/
        /// directory (or the book/xx directory if a language is provided).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Create a static version of the course.
    Build {
        /// ISO 639 language code (e.g. da for the Danish translation).
        #[arg(short, long)]
        language: Option<String>,

        /// Directory to place the build. If not provided, defaults to the book/
        /// directory (or the book/xx directory if a language is provided).
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn execute_task() -> Result<()> {
    let cli = Cli::parse();
    match cli.task {
        Task::InstallTools => install_tools(),
        Task::WebTests { dir } => run_web_tests(dir),
        Task::RustTests => run_rust_tests(),
        Task::Serve { language, output } => start_web_server(language, output),
        Task::Build { language, output } => build(language, output),
    }
}

/// Executes a command and returns an error if it fails.
fn run_command(cmd: &mut Command) -> Result<()> {
    let command_display = format!("{cmd:?}");
    println!("> {command_display}");
    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute command: {command_display}"))?;
    if !status.success() {
        let exit_description = if let Some(code) = status.code() {
            format!("exited with status code: {}", code)
        } else {
            "was terminated by a signal".to_string()
        };
        return Err(anyhow!("Command `{command_display}` {exit_description}"));
    }
    Ok(())
}

fn install_tools() -> Result<()> {
    println!("Installing project tools...");

    const PINNED_NIGHTLY: &str = "nightly-2025-09-01";

    // Install rustup components
    let rustup_steps = [
        ["toolchain", "install", "--profile", "minimal", PINNED_NIGHTLY],
        ["component", "add", "rustfmt", "--toolchain", PINNED_NIGHTLY],
    ];
    for args in rustup_steps {
        let mut cmd = Command::new("rustup");
        cmd.args(args);
        run_command(&mut cmd)?;
    }

    let cargo = env!("CARGO");
    // The --locked flag is important for reproducible builds.
    let tools = [
        ("mdbook", "0.4.52"),
        ("mdbook-svgbob", "0.2.2"),
        ("mdbook-pandoc", "0.10.4"),
        ("mdbook-i18n-helpers", "0.3.6"),
        ("i18n-report", "0.2.0"),
        ("mdbook-linkcheck2", "0.9.1"),
    ];

    for (tool, version) in tools {
        let mut cmd = Command::new(cargo);
        cmd.args(["install", tool, "--version", version, "--locked"]);
        run_command(&mut cmd)?;
    }

    // Install local tools from the workspace.
    let workspace_dir = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let local_tools = ["mdbook-exerciser", "mdbook-course"];
    for tool in local_tools {
        let mut cmd = Command::new(cargo);
        cmd.args(["install", "--path"])
            .arg(workspace_dir.join(tool))
            .arg("--locked");
        run_command(&mut cmd)?;
    }

    // Uninstall original linkcheck if currently installed (see issue no 2773)
    uninstall_mdbook_linkcheck()?;

    Ok(())
}

fn uninstall_mdbook_linkcheck() -> Result<()> {
    println!("Uninstalling old mdbook-linkcheck if installed...");
    let output = Command::new(env!("CARGO"))
        .args(["uninstall", "mdbook-linkcheck"])
        .output()
        .context("Failed to execute `cargo uninstall mdbook-linkcheck`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // This specific error is OK, it just means the package wasn't installed.
        if !stderr.contains("did not match any packages") {
            return Err(anyhow!(
                "Failed to uninstall `mdbook-linkcheck`.\n--- stderr:\n{stderr}"
            ));
        }
        println!("mdbook-linkcheck not installed. Continuing...");
    }
    Ok(())
}

fn run_web_tests(dir: Option<PathBuf>) -> Result<()> {
    println!("Running web tests...");
    let workspace_dir = Path::new(env!("CARGO_WORKSPACE_DIR"));

    let absolute_dir = dir.map(|d| d.canonicalize()).transpose()?;

    if let Some(d) = &absolute_dir {
        println!("Refreshing slide lists...");
        let refresh_slides_script = Path::new("tests")
            .join("src")
            .join("slides")
            .join("create-slide.list.sh");
        let mut cmd = Command::new(&refresh_slides_script);
        cmd.current_dir(workspace_dir).arg(d);
        run_command(&mut cmd)?;
    }

    let tests_dir = workspace_dir.join("tests");
    let mut cmd = Command::new("npm");
    cmd.current_dir(tests_dir).arg("test");

    if let Some(d) = absolute_dir {
        cmd.env("TEST_BOOK_DIR", d);
    }
    run_command(&mut cmd)
}

fn run_rust_tests() -> Result<()> {
    println!("Running rust tests...");
    let workspace_root = Path::new(env!("CARGO_WORKSPACE_DIR"));

    let mut cmd = Command::new("mdbook");
    cmd.current_dir(workspace_root).arg("test");
    run_command(&mut cmd)
}

fn run_mdbook_command(
    subcommand: &str,
    language: Option<String>,
    output_arg: Option<PathBuf>,
) -> Result<()> {
    let workspace_root = Path::new(env!("CARGO_WORKSPACE_DIR"));

    let mut cmd = Command::new("mdbook");
    cmd.current_dir(workspace_root).arg(subcommand);

    if let Some(language) = &language {
        println!("Language: {language}");
        cmd.env("MDBOOK_BOOK__LANGUAGE", language);
    }

    cmd.arg("-d");
    cmd.arg(get_output_dir(language, output_arg));

    run_command(&mut cmd)
}

fn start_web_server(
    language: Option<String>,
    output_arg: Option<PathBuf>,
) -> Result<()> {
    println!("Starting web server ...");
    run_mdbook_command("serve", language, output_arg)
}

fn build(language: Option<String>, output_arg: Option<PathBuf>) -> Result<()> {
    println!("Building course...");
    run_mdbook_command("build", language, output_arg)
}

fn get_output_dir(language: Option<String>, output_arg: Option<PathBuf>) -> PathBuf {
    // If the 'output' arg is specified by the caller, use that, otherwise output to
    // the 'book/' directory (or the 'book/xx' directory if a language was
    // specified).
    if let Some(d) = output_arg {
        d
    } else {
        Path::new("book").join(language.unwrap_or("".to_string()))
    }
}
