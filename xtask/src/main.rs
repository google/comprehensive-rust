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
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use walkdir::WalkDir;

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
    InstallTools {
        /// Use cargo-binstall for faster installation.
        #[arg(long)]
        binstall: bool,
    },
    /// Runs the web driver tests in the tests directory.
    WebTests {
        /// Optional 'book html' directory - if set, will also refresh the list
        /// of slides used by slide size test.
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },
    /// (Re)creates the slides.list.ts file based on the given book html
    /// directory.
    CreateSlideList {
        /// The book html directory
        #[arg(short, long)]
        dir: PathBuf,
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
        Task::InstallTools { binstall } => install_tools(binstall),
        Task::WebTests { dir } => run_web_tests(dir),
        Task::CreateSlideList { dir } => create_slide_list(dir),
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

fn install_tools(binstall: bool) -> Result<()> {
    println!("Installing project tools...");

    let cargo = env!("CARGO");

    let install_command = if binstall { "binstall" } else { "install" };

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
        cmd.args([install_command, tool, "--version", version, "--locked"]);
        run_command(&mut cmd)?;
    }

    // Install local tools from the workspace.
    let workspace_dir = Path::new(env!("CARGO_WORKSPACE_DIR"));
    // cargo-binstall does not support --path, so we always use cargo install here.
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
        create_slide_list(d.clone())?;
    }

    let tests_dir = workspace_dir.join("tests");
    let mut cmd = Command::new("npm");
    cmd.current_dir(tests_dir).arg("test");

    if let Some(d) = absolute_dir {
        cmd.env("TEST_BOOK_DIR", d);
    }
    run_command(&mut cmd)
}

// Creates a list of .html slides from the html directory containing the
// index.html to check the slides.
// - CI environment: Only modified files are listed
// - Otherwise: All existing html files
fn create_slide_list(html_directory: PathBuf) -> Result<()> {
    let workspace_dir = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let tests_dir = workspace_dir.join("tests");

    // Check if the provided directory is correct
    if !html_directory.join("index.html").exists() {
        return Err(anyhow!(
            "Could not find index.html in {}. Please check if the correct directory is used (e.g. book/html).",
            html_directory.display()
        ));
    }

    // These special slides are not checked against the style guide
    let exclude_paths = [
        "exercise.html",
        "solution.html",
        "toc.html",
        "print.html",
        "404.html",
        "glossary.html",
        "index.html",
        "course-structure.html",
    ]
    .map(PathBuf::from);

    // Collect the files relevant for evaluation.
    // - CI environment variable is set: all modified markdown files in the src/
    //   directory
    // - all html files in the provided directory otherwise
    let candidate_slides: Vec<PathBuf> = if env::var("CI").is_ok() {
        println!("CI environment detected, checking only modified slides.");
        // GITHUB_BASE_REF is available in PRs. Default to 'main' for other CI
        // contexts.
        let base_ref = env::var("GITHUB_BASE_REF").unwrap_or("main".to_string());
        let mut cmd = Command::new("git");
        cmd.arg("diff")
            .arg("--name-only")
            .arg(format!("{}...", base_ref))
            .arg("--")
            // Retrieve all modified files in the src directory.
            // Pathspec syntax: https://git-scm.com/docs/gitglossary#Documentation/gitglossary.txt-pathspec
            // `*` can match path separators, thus matches also files in
            // subdirectories
            .arg("src/*.md");
        println!("> {cmd:?}");
        let output = cmd.output().context("Failed to run git diff")?;
        String::from_utf8(output.stdout)?
            .lines()
            .map(|line| {
                let path = Path::new(line);
                // We know the path starts with "src/" because of the pathspec in the
                // `git diff` command, and we need it relative to the html base
                // directory
                let stripped_path = path.strip_prefix("src").unwrap();
                let mut html_path = stripped_path.to_path_buf();
                // replace the .md extension with .html
                html_path.set_extension("html");
                html_path
            })
            .collect()
    } else {
        println!("Local environment, checking all slides.");
        WalkDir::new(&html_directory)
            .into_iter()
            .filter_map(|e| e.ok())
            // only files with .html extension
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "html"))
            // relative path inside the html directory
            .map(|e| e.path().strip_prefix(&html_directory).unwrap().to_path_buf())
            .collect()
    };
    // Filter the candidate slides
    let mut slides = Vec::new();
    for slide in candidate_slides {
        // Skip excluded files
        if exclude_paths.iter().any(|exclude_path| slide.ends_with(exclude_path)) {
            continue;
        }

        // Test if the html files actually exist
        let full_path = html_directory.join(&slide);
        if !full_path.exists() {
            continue;
        }

        // Optimization: check if these are redirection html files and skip these
        let content = fs::read_to_string(&full_path)
            .with_context(|| format!("Failed to read slide: {}", slide.display()))?;
        if content.contains("Redirecting to...") {
            continue;
        }
        slides.push(slide);
    }

    if env::var("CI").is_ok() {
        println!("The following slides have been modified and will be checked:");
        for slide in &slides {
            println!("{}", slide.display());
        }
    }

    // Write the file list into a .ts file that can be read by the JS based webtest
    let output_path = tests_dir.join("src").join("slides").join("slides.list.ts");
    let mut output_content = "export const slides = [\n".to_string();
    for slide in slides {
        output_content.push_str(&format!("  \"{}\",\n", slide.display()));
    }
    output_content.push_str("];\n");

    fs::write(&output_path, output_content)
        .with_context(|| format!("Failed to write to {}", output_path.display()))?;

    Ok(())
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
