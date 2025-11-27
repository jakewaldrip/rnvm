use clap::{Command, Subcommand, error::ErrorKind};

#[derive(Subcommand)]
pub enum Commands {
    /// Handles new version install
    Install {
        #[arg(index = 1)]
        version_num: String,
    },
    /// Prints current version
    Current,
    /// Switch to an installed version
    Use {
        #[arg(index = 1)]
        version_num: String,
    },
}

pub fn handle_install_command(version_num: &str, cmd: &mut Command) {
    // TODO: throw this same error if the version does not exist
    // Add instructions to fix
    if version_num == "123" {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Version {version_num} does not exist"),
        )
        .exit()
    }

    // TODO: do installing stuff here
    println!("Installing {version_num}");
}

pub fn handle_current_command(cmd: &mut Command) {
    // TODO: check if dir exists with any installed node versions
    // Check if our metadata is pointing to one to throw this error
    // Add instructions to fix
    if true {
        cmd.error(ErrorKind::DisplayHelp, "No currently installed versions")
            .exit()
    }

    // TODO: print version number here
    println!("Current version: 12.0.1")
}

pub fn handle_use_command(version_num: &str, cmd: &mut Command) {
    // TODO: check if dir exists with the requested node version to throw this error
    // Add instructions to fix
    if version_num == "123" {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Version {version_num} is not installed"),
        )
        .exit()
    }

    // TODO: switch version here
    println!("Switching version to {version_num}")
}
