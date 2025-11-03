# Project Overview

This repository contains the source code for Comprehensive Rust, a family of
courses on Rust developed by Google, starting with Rust foundations, and
including deep dives into specialized topics like Android, Chromium, bare-metal
development, and concurrency. The project is a Rust workspace that leverages
`mdbook` to generate a course website.

## Key Technologies

- **Rust:** The primary programming language for the course subject, custom
  tools, and examples.
- **mdbook:** A command-line tool to create books from Markdown files, used for
  generating the course website.
- **Custom mdbook Preprocessors:** `mdbook-course` and `mdbook-exerciser` are
  Rust binaries that extend `mdbook`'s functionality, for example, to extract
  exercise starter code.
- **`cargo xtask`:** A custom binary within the workspace used for project
  automation, simplifying common development tasks.

# Building and Running

The project uses `cargo xtask` for project-specific automation, like builds,
tests, and managing translations.

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
   This is a necessary first step for working with this repository. It will
   install the correct versions of all tools used by the project.

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
  to `rustfmt.toml` and `dprint.json`. Note that you must first install the
  project tools with `cargo xtask install-tools`.
- **Contributions:** Refer to `CONTRIBUTING.md` for guidelines on contributing
  to the project.
- **Style:** Refer to `STYLE.md` for style guidelines. When making changes to
  Markdown files in `src/`, always first read `STYLE.md` and follow its
  conventions.
- **GitHub Actions:** The project uses composite GitHub Actions to simplify CI
  workflows. These actions should be preferred over hand-written commands.
  - **`apt-get-install`:** This action efficiently installs Debian packages. It
    configures `dpkg` and `apt` to skip documentation and translations, and
    ensures that `apt-get update` is run only once per job. This significantly
    speeds up CI runs.
  - **`install-mdbook`:** A composite action to install `mdbook` and its
    dependencies, including `pandoc` and `texlive`.
  - **`setup-rust-cache`:** A composite action that configures the
    `Swatinem/rust-cache` action.

## Markdown Conventions

- **Headings:**
  - **H1 (`#`):** Used for the main title of each page. Each slide has exactly
    one title.
  - **H2 (`##`):** Used for major sections. Slides do not use H2 headings to
    save vertical space; more slides are created instead.
  - **H3 (`###`):** Used for sub-sections, but not on slides.

- **Emphasis:**
  - **Bold (`**...**`):** Used to highlight key terms, commands, and for notes
    (e.g., `**Note:**`). The colon (`:`) is included inside the bold text for
    notes.
  - **Italic (`_..._`):** Used for general emphasis, titles of external
    articles, and for terms being defined.

- **Code:**
  - **Inline Code (`` `...` ``):** Used for code snippets, file names, commands,
    type names, and language keywords. Rust fragments are formatted as `rustfmt`
    would.
  - **Code Blocks (`` ```...``` ``):** Fenced code blocks are used for
    multi-line code examples, annotated with a language identifier (e.g.,
    `rust`, `c`, `ignore`).
  - **Interactive Code Blocks:** Rust examples are made interactive with
    `editable`. Examples that fail to compile are marked with `compile_fail` or
    `should_panic`.
  - **Diagrams:** The `bob` language identifier is used in code blocks to
    generate ASCII art diagrams.
  - **Formatting Control:** The `#[rustfmt::skip]` attribute is used to prevent
    `rustfmt` from formatting specific code blocks, though it is used rarely.

- **Lists:**
  - **Bulleted Lists:** Unordered lists are the primary way to lay out key
    points on slides.
  - **Glossary Format:** The glossary uses a specific format with a colon and
    backslash (`:\`) after each term to create a hard line break for visual
    formatting.

- **Other Markdown Elements:**
  - **Block Quotes (`> ...`):** Used sparingly for important notes, warnings, or
    supplementary information to draw attention.
  - **Links:** Both standard (`[text](url)`) and reference-style
    (`[text][label]`) links are used.
  - **Tables:** Markdown tables are used to present structured data.
  - **Horizontal Rules (`---`):** Not used on slides.

- **HTML Tags:**
  - **`<details>`:** Used for collapsible "speaker notes".
  - **`<kbd>`:** Used to denote keyboard keys. Each key in a combination must be
    wrapped in its own tag, e.g., `<kbd>Ctrl</kbd> + <kbd>S</kbd>`.
  - **`<style>`:** Used rarely for targeted custom CSS.
  - **`<img>`:** Used to embed images.

- **Project-Specific Conventions:**
  - **mdbook Includes:** The `{{#include ...}}` helper is used to include
    snippets from other files.
  - **mdbook Segments:** The `{{%segment ...%}}` and `{{%session ...%}}` helpers
    are used for course structuring.
  - **Frontmatter:** Files start with YAML frontmatter (enclosed in `---`) to
    provide metadata.
  - **Doc Comments:** Standard Rust doc comments (`///`, `//!`) are used.
    `/// # Safety` is used to document safety preconditions for `unsafe` code.
  - **Comments:** HTML comments (`<!-- ... -->`) are used for editor/translator
    instructions and content control (e.g., `mdbook-xgettext: skip`).

# Project-Specific Technical Context

This section contains critical, non-obvious technical details about this
project's tooling and environment that an AI assistant needs to know to perform
its tasks correctly.

## `mdbook` Behavior

- **Isolated Code Snippets:** `mdbook` treats each fenced Rust code block (e.g.,
  `` ```rust ... ``` ``) as a separate compilation unit. When analyzing a code
  snippet, treat it as a self-contained program. Do not assume it shares a scope
  or context with other snippets in the same file unless the surrounding text
  explicitly states otherwise.

## Interacting with the `mdbook` Theme

The `mdbook` theme has several interactive elements. Here's how to interact with
them:

- **Sidebar Toggle:** The sidebar can be opened and closed by clicking the
  "hamburger" button in the top-left of the body text. This button has the ID
  `sidebar-toggle`. You can use the following JavaScript to toggle the sidebar:

  ```javascript
  const button = document.getElementById("sidebar-toggle");
  button.click();
  ```

## WebdriverIO Testing

This project uses WebdriverIO for browser-based integration tests. Here are some
key findings about the test environment:

### Test Environments

The `tests/` directory contains two primary configurations:

- `npm test` (runs `wdio.conf.ts`): This is the standard for self-contained
  integration tests. It uses `@wdio/static-server-service` to create a temporary
  web server on port 8080.
- `npm run test-mdbook` (runs `wdio.conf-mdbook.ts`): This is for testing
  against a live `mdbook serve` instance, which typically runs on port 3000.

It is important to use the standard `npm test` command for most test development
to ensure the tests are self-contained.

### Writing Stable Tests

Tests can be flaky if they don't correctly handle the asynchronous nature of the
web browser and the test environment's state management.

- **State Leakage Between Tests:** Despite what the WebdriverIO documentation
  might suggest, `browser.url()` is not always sufficient to guarantee a clean
  slate between tests. Lingering state, such as inline CSS styles applied by
  JavaScript, can leak from one test into the next, causing unexpected failures.
  The most effective solution found for this project is to add
  `await browser.refresh();` to the `beforeEach` hook. This forces a full page
  reload that properly clears the old state.

- **Race Conditions with Dynamic Elements:** Many elements in this project are
  created dynamically by JavaScript after the initial page load. If a test tries
  to access an element immediately after navigation, it may fail because the
  script hasn't finished running and the element doesn't exist in the DOM yet.
  This creates a race condition. To prevent this, always use
  `await element.waitForExist()` to ensure the element is present before trying
  to interact with it or assert its state (e.g., `toBeDisplayed()`).

### Handling Redirects

`mdbook` uses a redirect map defined in `book.toml` under the
`[output.html.redirect]` section. When writing tests, it is crucial to use the
final, non-redirecting URL for navigation. Navigating to a URL that is a
redirect will cause the browser to follow it, but this process can strip URL
query parameters, leading to test failures for features that depend on them.

### Running and Debugging Tests

To run a single test file, use the `--spec` flag with the a string matching the
file name:

```bash
npm test -- --spec redbox
```

To check for flakiness, you can repeat a test multiple times using the
`--repeat` flag:

```bash
npm test -- --spec redbox --repeat 100
```

Use `--mochaOpts.grep` to run a single test within a file:

```bash
npm test -- --spec redbox --mochaOpts.grep "should be hidden by default"
```
