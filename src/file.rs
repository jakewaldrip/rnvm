use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;

pub fn get_active_version_from_metadata() -> Result<String, Error> {
    let rnvm_dir = env::var("RNVM_DIR")
        .map_err(|_| Error::new(std::io::ErrorKind::NotFound, "RNVM_DIR not set"))?;
    let metadata_path = Path::new(&rnvm_dir).join("metadata");

    let content = if metadata_path.exists() {
        fs::read_to_string(&metadata_path)?
    } else {
        fs::write(&metadata_path, "")?;
        String::new()
    };

    Ok(content.trim().to_string())
}

pub fn set_active_version_in_metadata(active_version_num: &str) -> Result<(), Error> {
    let rnvm_dir = env::var("RNVM_DIR")
        .map_err(|_| Error::new(std::io::ErrorKind::NotFound, "RNVM_DIR not set"))?;
    let metadata_path = Path::new(&rnvm_dir).join("metadata");

    fs::write(&metadata_path, active_version_num)?;
    Ok(())
}

pub fn get_installed_versions() -> Result<Vec<String>, Error> {
    let rnvm_dir = std::env::var("RNVM_DIR")
        .map_err(|_| Error::new(std::io::ErrorKind::NotFound, "RNVM_DIR not set"))?;

    let mut versions = Vec::new();
    for entry in std::fs::read_dir(rnvm_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        if let Some(name) = entry.file_name().to_str() {
            versions.push(name.to_string());
        }
    }
    Ok(versions)
}

pub fn does_installed_version_exist(version_number: &str) -> Result<bool, Error> {
    let rnvm_dir = std::env::var("RNVM_DIR")
        .map_err(|_| Error::new(std::io::ErrorKind::NotFound, "RNVM_DIR not set"))?;
    let version_path = Path::new(&rnvm_dir).join(version_number);

    Ok(version_path.exists())
}

pub fn remove_installed_version(version_number: &str) -> Result<(), Error> {
    let rnvm_dir = std::env::var("RNVM_DIR")
        .map_err(|_| Error::new(std::io::ErrorKind::NotFound, "RNVM_DIR not set"))?;
    let version_path = Path::new(&rnvm_dir).join(version_number);

    fs::remove_dir_all(version_path)?;
    Ok(())
}
