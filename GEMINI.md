# Project Overview

This repository contains the source code for Comprehensive Rust, a family of
courses on Rust developed by Google. The project is a Rust workspace that
leverages `mdbook` to generate a comprehensive course website, including various
deep dives into specialized topics like Android, Chromium, bare-metal
development, and concurrency.

## Key Technologies

- **Rust:** The primary programming language for the course content, custom
  tools, and examples.
- **mdbook:** A command-line tool to create books from Markdown files, used for
  generating the course website.
- **Custom mdbook Preprocessors:** `mdbook-course` and `mdbook-exerciser` are
  Rust binaries that extend `mdbook`'s functionality, for example, to extract
  exercise starter code.
- **`cargo xtask`:** A custom binary within the workspace used for project
  automation, simplifying common development tasks.

# Building and Running

The project uses `cargo xtask` for all build and development automation.

## Setup

1. **Install Rust:** Follow the instructions on
   [https://rustup.rs/](https://rustup.rs/).
2. **Clone Repository:**
   ```bash
   git clone https://github.com/google/comprehensive-rust/
   cd comprehensive-rust
   ```
3. **Install Project Tools:**
   ```bash
   cargo xtask install-tools
   ```

## Commands

All commands are run using `cargo xtask`. Run `cargo xtask --help` for a full
list of options.

- **Serve the Course Locally:** Starts a web server to view the course content.
  ```bash
  cargo xtask serve [--language <ISO_639_language_code>] [--output <output_directory>]
  ```
  (e.g., `cargo xtask serve -l da` for the Danish translation)

- **Build the Course:** Creates a static version of the course in the `book/`
  directory.
  ```bash
  cargo xtask build [--language <ISO_639_language_code>] [--output <output_directory>]
  ```

- **Run Rust Snippet Tests:** Tests all Rust code snippets included in the
  course material.
  ```bash
  cargo xtask rust-tests
  ```

- **Run Web Driver Tests:** Executes web driver tests located in the `tests/`
  directory.
  ```bash
  cargo xtask web-tests [--dir <book_html_directory>]
  ```

# Development Conventions

- **Project Automation:** `cargo xtask` is the primary interface for common
  development tasks.
- **Course Content:** Markdown files in the `src/` directory, structured
  according to `src/SUMMARY.md`.
- **Code Formatting:** `dprint fmt` is used to format all source files according
  to `rustfmt.toml` and `dprint.json`.
- **Contributions:** Refer to `CONTRIBUTING.md` for guidelines on contributing
  to the project.
- **Style:** Refer to `STYLE.md` for style guidelines.
