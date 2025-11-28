use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

use indicatif::{ProgressBar, ProgressStyle};
use tar::Archive;
use ureq::Error;
use xz2::read::XzDecoder;

pub trait InstallStrategy {
    fn install(&self, version_num: &str) -> Result<(), Error>;
}

pub struct LinuxClient {}

impl InstallStrategy for LinuxClient {
    fn install(&self, version_num: &str) -> Result<(), Error> {
        // Checking these in the parent, so they should never error
        let rnvm_dir = env::var("RNVM_DIR").expect("RNVM_DIR not set");
        let out_dir = PathBuf::from(rnvm_dir).join(version_num);
        if Path::new(&out_dir).exists() {
            println!("Node v{version_num} already installed");
            return Ok(());
        }

        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::with_template("{spinner} Downloading from Nodejs...")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        spinner.enable_steady_tick(Duration::from_millis(100));

        let url =
            format!("https://nodejs.org/dist/v{version_num}/node-v{version_num}-linux-x64.tar.xz");
        let response = ureq::get(&url).call();

        let mut reader = match response {
            Ok(resp) => resp.into_body().into_reader(),
            Err(e) => return Err(e),
        };

        let tmp_dir = out_dir.with_extension("tmp");
        fs::create_dir_all(&tmp_dir)?;

        let decompressor = XzDecoder::new(&mut reader);
        let mut archive = Archive::new(decompressor);
        archive.unpack(&tmp_dir)?;

        // Move the unzipped file into the output directory
        let mut entries = fs::read_dir(&tmp_dir)?
            .filter_map(Result::ok)
            .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false));

        if let Some(first_dir) = entries.next() {
            fs::rename(first_dir.path(), &out_dir)?;
        }
        fs::remove_dir_all(&tmp_dir)?;

        Ok(())
    }
}

pub struct MacSiliconClient {}

// TODO: impl mac install
impl InstallStrategy for MacSiliconClient {
    fn install(&self, _version_num: &str) -> Result<(), Error> {
        println!("Mac install not yet implemented");
        Ok(())
    }
}
