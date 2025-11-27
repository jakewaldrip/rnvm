use clap::{CommandFactory, Parser};

use crate::commands::{
    Commands, handle_current_command, handle_install_command, handle_use_command,
};

mod commands;

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
        None => {}
    }
}
