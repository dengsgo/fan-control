use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "wiiznokes", version, about = "fan control app", long_about = None)]
pub struct Args {
    #[arg(short = 'p', long = "path", id = "path to settings/config directory")]
    pub config_dir_path: Option<PathBuf>,
}

impl Args {
    pub fn validate_config_dir_path(path: &Path) -> Result<(), String> {
        if !path.is_dir() {
            return Err(format!("{} is not a directory", path.display()));
        }

        match path.read_dir() {
            Ok(mut entries) => {
                if entries.next().is_none() {
                    Ok(())
                } else {
                    Err(format!("{} directory is not empty", path.display()))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}