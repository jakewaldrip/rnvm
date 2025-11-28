use std::env;
use std::env::JoinPathsError;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::path::PathBuf;

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

/// Create a path that removes all existing rnvm node versions and prepend ours
/// Prints to stdout so we can pipe it into an eval in the shell
pub fn update_path(version_num: &str) -> Result<(), JoinPathsError> {
    let path = env::var("PATH").expect("PATH is not set");
    let rnvm_dir = env::var("RNVM_DIR").expect("RNVM_DIR is not set");

    let mut paths: Vec<PathBuf> = env::split_paths(&path)
        .filter(|p| !p.to_string_lossy().contains(&rnvm_dir.clone()))
        .collect();

    let rnvm_version_path = Path::new(&rnvm_dir).join(version_num).join("bin");
    paths.insert(0, rnvm_version_path);

    let new_path = env::join_paths(paths)?;
    println!("export PATH=\"{}\"", new_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_get_active_version_from_metadata() {
        // Set up temp dir
        let temp_dir = env::temp_dir().join("rnvm_test");
        fs::create_dir_all(&temp_dir).unwrap();
        unsafe {
            env::set_var("RNVM_DIR", temp_dir.to_str().unwrap());
        }

        let result = get_active_version_from_metadata().unwrap();
        assert_eq!(result, "");

        // Write and test
        fs::write(temp_dir.join("metadata"), "v18.17.0").unwrap();
        let result = get_active_version_from_metadata().unwrap();
        assert_eq!(result, "v18.17.0");

        // Cleanup
        fs::remove_dir_all(&temp_dir).unwrap();
        unsafe {
            env::remove_var("RNVM_DIR");
        }
    }
}
