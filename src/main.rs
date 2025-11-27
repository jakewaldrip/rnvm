use std::{env, fs, path::Path};

use clap::{CommandFactory, Parser, error::ErrorKind};

use crate::commands::{
    Commands, handle_current_command, handle_install_command, handle_list_command,
    handle_remove_command, handle_use_command,
};

mod commands;
mod file;

/// Manages node installations, but blazingly fast
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let args = Args::parse();
    let mut cmd = Args::command();

    // Ensure working directory exists, create if needed
    // TODO: add better instructions to error message
    let Ok(rnvm_dir) = env::var("RNVM_DIR") else {
        cmd.error(ErrorKind::InvalidValue, "RNVM_DIR not found")
            .exit()
    };

    let path = Path::new(&rnvm_dir);
    if !path.exists()
        && let Err(_) = fs::create_dir(path)
    {
        cmd.error(
            ErrorKind::Io,
            format!("Failed to create directory {path:?}"),
        )
        .exit()
    } else if !path.is_dir() {
        cmd.error(
            ErrorKind::Io,
            format!("{path:?} exists but is not a directory"),
        )
        .exit()
    }

    match &args.command {
        Some(Commands::Install { version_num }) => {
            handle_install_command(version_num, &mut cmd);
        }
        Some(Commands::Current) => {
            handle_current_command(&mut cmd);
        }
        Some(Commands::Use { version_num }) => {
            handle_use_command(version_num, &mut cmd);
        }
        Some(Commands::Remove { version_num }) => {
            handle_remove_command(version_num, &mut cmd);
        }
        Some(Commands::List) => {
            handle_list_command(&mut cmd);
        }
        None => {}
    }
}
