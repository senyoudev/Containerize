use std::process::Output;

use anyhow::{Context, Result};

fn execute_command(command: &str, args: &[String]) -> Result<std::process::Output> {
    std::process::Command::new(command)
        .args(args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, args
            )
        })
}

fn print_output(output: &Output) -> Result<()> {
    if output.status.success() {
        let std_out = std::str::from_utf8(&output.stdout)?;
        print!("{}", std_out);
        let std_err = std::str::from_utf8(&output.stderr)?;
        eprint!("{}", std_err);
    } else {
        std::process::exit(1);
    }

    Ok(())
}

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...");
        std::process::exit(1);
    }

    let command = &args[3];
    let command_args = &args[4..];

    let output = execute_command(command, command_args)?;
    let _ = print_output(&output);

    Ok(())
}
