use std::{env, process::Command};

type DynError = Box<dyn std::error::Error>;

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
        _ => {
            return Err(Box::from(get_help_string(task.as_deref())));
        }
    }
    Ok(())
}

fn install_tools() -> Result<(), DynError> {
    println!("Installing project tools...");

    let install_args: Vec<Vec<&str>> = vec![
        // The --locked flag is important for reproducible builds. It also
        // avoids breakage due to skews between mdbook and mdbook-svgbob.
        vec!["mdbook", "--locked", "--version", "0.4.44"],
        vec!["mdbook-svgbob", "--locked", "--version", "0.2.1"],
        vec!["mdbook-pandoc", "--locked", "--version", "0.9.3"],
        vec!["mdbook-i18n-helpers", "--locked", "--version", "0.3.5"],
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
                "cargo install {} {} exited with status code: {}",
                args.get(0).unwrap(),
                args.get(1).unwrap(),
                status.code().unwrap()
            );
            return Err(Box::from(error_message));
        }
    }

    Ok(())
}

fn get_help_string(task: Option<&str>) -> String {
    if let Some(t) = task {
        format!(
            "Unrecognized task '{t}'. Available tasks:

install-tools            Installs the tools the project depends on.
"
        )
    } else {
        "Missing task. To execute a task run `cargo xtask [task]`.".to_string()
    }
}
