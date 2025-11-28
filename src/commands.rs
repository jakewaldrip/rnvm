use clap::{Command, Subcommand, error::ErrorKind};

use crate::{
    client::InstallStrategy,
    file::{
        does_installed_version_exist, get_active_version_from_metadata, get_installed_versions,
        remove_installed_version, set_active_version_in_metadata, update_path,
    },
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
    /// Set the path on shell start
    #[command(hide = true)]
    Start,
}

pub fn handle_install_command(
    version_num: &str,
    install_client: &dyn InstallStrategy,
    cmd: &mut Command,
) {
    println!("Installing Node v{version_num}");
    let Ok(()) = install_client.install(version_num).map_err(|_| {
        cmd.error(
            ErrorKind::ValueValidation,
            format!("Node v{version_num} does not exist"),
        )
        .exit()
    });
    println!("Successfully installed Node v{version_num}");
}

pub fn handle_current_command(cmd: &mut Command) {
    let Ok(active_version) = get_active_version_from_metadata() else {
        cmd.error(ErrorKind::Io, "Failed to read metadata").exit()
    };

    if active_version.is_empty() {
        cmd.error(ErrorKind::ValueValidation, "No active version set")
            .exit()
    }

    println!("Using Node v{active_version}");
}

pub fn handle_use_command(version_num: &str, cmd: &mut Command) {
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

    let Ok(()) = update_path(version_num) else {
        cmd.error(ErrorKind::ValueValidation, "Failed to update $PATH")
            .exit()
    };

    set_active_version_in_metadata(version_num)
        .map_err(|_| {
            cmd.error(ErrorKind::Io, "Failed to write to .rnvm directory")
                .exit()
        })
        .unwrap();
}

pub fn handle_remove_command(version_num: &str, cmd: &mut Command) {
    let Ok(active_version) = get_active_version_from_metadata() else {
        cmd.error(ErrorKind::Io, "Failed to read active version")
            .exit()
    };

    if active_version == version_num {
        cmd.error(
            ErrorKind::ValueValidation,
            "Cannot remove the version that is currently used",
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
    println!("Successfully removed Node v{version_num}!");
}

pub fn handle_list_command(cmd: &mut Command) {
    let Ok(installed_versions) = get_installed_versions() else {
        cmd.error(ErrorKind::Io, "Failed to get installed versions")
            .exit()
    };

    if installed_versions.is_empty() {
        println!("No installed versions");
    } else {
        println!("{} installed versions", installed_versions.len());
        for version in &installed_versions {
            println!("Node v{version}");
        }
    }
}

pub fn handle_start_command(cmd: &mut Command) {
    let Ok(active_version) = get_active_version_from_metadata() else {
        cmd.error(ErrorKind::Io, "Failed to read active version")
            .exit()
    };

    let Ok(()) = update_path(&active_version) else {
        cmd.error(ErrorKind::ValueValidation, "Failed to update $PATH")
            .exit()
    };
}
