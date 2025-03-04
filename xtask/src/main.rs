use std::{env, process::Command};

type DynError = Box<dyn std::error::Error>;

const INSTALLATION_ARGS: [[&str; 4]; 7] = [
    // The --locked flag is important for reproducible builds. It also
    // avoids breakage due to skews between mdbook and mdbook-svgbob.
    ["mdbook", "--locked", "--version", "0.4.44"],
    ["mdbook-svgbob", "--locked", "--version", "0.2.1"],
    ["mdbook-pandoc", "--locked", "--version", "0.9.3"],
    ["mdbook-i18n-helpers", "--locked", "--version", "0.3.5"],
    ["i18n-report", "--locked", "--version", "0.2.0"],
    // These packages are located in this repository
    ["--path", "mdbook-exerciser", "--locked", ""],
    ["--path", "mdbook-course", "--locked", ""],
];

fn main() {
    if let Err(e) = execute_task() {
        eprintln!("{e}");
        std::process::exit(-1);
    }
}

fn execute_task() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("install-tools") => install_tools()?,
        _ => print_help(task.as_deref()),
    }
    Ok(())
}

fn install_tools() -> Result<(), DynError> {
    println!("Installing project tools...");

    for args in INSTALLATION_ARGS.iter() {
        Command::new(env!("CARGO"))
            .arg("install")
            .args(args.iter().filter(|a| **a != ""))
            .status()?;
    }

    Ok(())
}

fn print_help(task: Option<&str>) {
    if let Some(t) = task {
        eprintln!(
            "Unrecognized task '{t}'. Available tasks:

install-tools            Installs the tools the project depends on.

Run with `cargo xtask [task]`.
"
        );
    } else {
        eprintln!("Missing task. To execute a task run `cargo xtask [task]`.");
    }
}
