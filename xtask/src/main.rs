use std::{
    env,
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = execute_task()  {
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

fn install_tools() -> Result<(), DynError>{
    println!("Installing project tools...");
    // The --locked flag is important for reproducible builds. It also
    // avoids breakage due to skews between mdbook and mdbook-svgbob.
    Command::new(env!("CARGO")).arg("install").args(["mdbook", "--locked", "--version", "0.4.44"]).status()?;
    Command::new(env!("CARGO")).arg("install").args(["mdbook-svgbob", "--locked", "--version", "0.2.1"]).status()?;
    Command::new(env!("CARGO")).arg("install").args(["mdbook-pandoc", "--locked", "--version", "0.9.3"]).status()?;
    Command::new(env!("CARGO")).arg("install").args(["mdbook-i18n-helpers", "--locked", "--version", "0.3.5"]).status()?;
    Command::new(env!("CARGO")).arg("install").args(["i18n-report", "--locked", "--version", "0.2.0"]).status()?;
    // These packages are located in this repository
    Command::new(env!("CARGO")).arg("install").args(["--path", "mdbook-exerciser", "--locked"]).status()?;
    Command::new(env!("CARGO")).arg("install").args(["--path", "mdbook-course", "--locked"]).status()?;
    Ok(())
}

fn print_help(task: Option<&str>) {
    if let Some(t) = task {
    eprintln!(
        "Unrecognized task '{t}'. Available tasks:

install-tools            Installs the tools the project dependends on.

Run with `cargo xtask [task]`.
");
    } else {
        eprintln!("Missing task. To execute a task run `cargo xtask [task]`.");
    }
}