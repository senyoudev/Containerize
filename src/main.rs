use std::process::Stdio;
use anyhow::{Context, Result};
use tempfile::tempdir;
use std::os::unix::fs;

const CLONE_NEWPID: i32 = 0x20000000;

extern "C" {
    fn unshare(flags: i32) -> i32;
}

fn execute_command(command: &str, args: &[String]) -> Result<i32> {
    let status = std::process::Command::new(command)
        .args(args)
        .stdout(Stdio::inherit()) 
        .stderr(Stdio::inherit())  
        .status()  
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, args
            )
        })?;

    Ok(status.code().unwrap_or(1))
}

fn create_pid_namespace() -> Result<()> {
    unsafe {
        if unshare(CLONE_NEWPID) == -1 {
            return Err(anyhow::anyhow!("Failed to unshare PID namespace: {}", std::io::Error::last_os_error()));
        }
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

    let tmp_dir = tempdir()?;
    let to = tmp_dir.path().join(command.strip_prefix("/").unwrap_or(command));
    std::fs::create_dir_all(to.parent().unwrap())?;
    std::fs::copy(command, to)?;

    let dev_null = tmp_dir.path().join("dev/null");
    std::fs::create_dir_all(dev_null.parent().unwrap())?;
    std::fs::File::create(dev_null)?;

    create_pid_namespace()?;
    fs::chroot(tmp_dir.path())?;

    let exit_code = execute_command(&command, command_args)?;
    std::process::exit(exit_code);
}
