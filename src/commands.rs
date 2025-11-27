use clap::{Command, Subcommand, error::ErrorKind};

use crate::file::{
    does_installed_version_exist, get_active_version_from_metadata, get_installed_versions,
    remove_installed_version, set_active_version_in_metadata,
};

#[derive(Subcommand)]
pub enum Commands {
    /// Install new version
    Install {
        #[arg(index = 1)]
        version_num: String,
    },
    /// Print current version
    Current,
    /// Switch to an installed version
    Use {
        #[arg(index = 1)]
        version_num: String,
    },
    /// Remove an installed version
    Remove {
        #[arg(index = 1)]
        version_num: String,
    },
    /// List all installed versions
    List,
}

// TODO: finish
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
    let Ok(active_version) = get_active_version_from_metadata() else {
        cmd.error(ErrorKind::Io, "Failed to read metadata").exit()
    };

    // TODO: better instructions
    if active_version.is_empty() {
        cmd.error(ErrorKind::ValueValidation, "No active version set")
            .exit()
    }

    println!("Using Node v{active_version}")
}

// TODO: finish
pub fn handle_use_command(version_num: &str, cmd: &mut Command) {
    // TODO: Add instructions to fix
    let Ok(does_version_exist) = does_installed_version_exist(version_num) else {
        cmd.error(ErrorKind::Io, "Failed to read rnvm directory")
            .exit()
    };

    // TODO: better instructions
    if !does_version_exist {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Node v{version_num} is not installed"),
        )
        .exit()
    }

    // BIG TODO: adjust path to actually use the associated version

    println!("Switching to Node v{version_num}");
    set_active_version_in_metadata(version_num)
        .map_err(|_| {
            cmd.error(ErrorKind::Io, "Failed to write to .rnvm directory")
                .exit()
        })
        .unwrap();
    println!("Now using Node v{version_num}")
}

pub fn handle_remove_command(version_num: &str, cmd: &mut Command) {
    let Ok(active_version) = get_active_version_from_metadata() else {
        cmd.error(ErrorKind::Io, "Failed to read active version")
            .exit()
    };

    if active_version == version_num {
        cmd.error(
            ErrorKind::ValueValidation,
            "Cannot remove version that is currently used",
        )
        .exit()
    }

    let Ok(does_version_exist) = does_installed_version_exist(version_num) else {
        cmd.error(ErrorKind::Io, "Failed to read rnvm directory")
            .exit()
    };

    if !does_version_exist {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Node v{version_num} is not installed"),
        )
        .exit()
    }

    println!("Removing Node v{version_num}");
    remove_installed_version(version_num)
        .map_err(|_| {
            cmd.error(
                ErrorKind::ValueValidation,
                format!("Failed to remove Node v{version_num}"),
            )
            .exit()
        })
        .unwrap();
    println!("Removed Node v{version_num}");
}

pub fn handle_list_command(cmd: &mut Command) {
    // TODO: better error message
    let Ok(installed_versions) = get_installed_versions() else {
        cmd.error(ErrorKind::Io, "Failed to get installed versions from path")
            .exit()
    };

    println!("All installed versions");
    for version in &installed_versions {
        println!("Node v{}", version);
    }
}
