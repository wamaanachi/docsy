use std::io::Error;
use std::path::{Path, PathBuf};
use cargo_toml::Manifest;
use clap::Parser;
use crate::cli::CliArgs;

pub(crate) struct CrateCheck {
    dir: PathBuf,
}

impl CrateCheck {
    pub(crate) fn new() -> Self {
        let dir: PathBuf = if CliArgs::parse().path.is_some() {
            PathBuf::from(Path::new(&CliArgs::parse().path.unwrap()))
        } else {
            std::env::current_dir().unwrap()
        };

        Self {
            dir
        }
    }

    /// Checks if the current directory is an active rust repo
    ///
    /// Checks for a Cargo.toml, otherwise returns error
    pub(crate) fn directory(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir: PathBuf = self.dir.clone();

        if !current_dir.is_dir() || !current_dir.exists() {
            return Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "Directory not found")))
        }

        let cargo_toml = current_dir.join("Cargo.toml");

        if cargo_toml.exists() {
            Ok(())
        } else {
            Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "Cargo.toml")))
        }
    }

    /// Checks the name of the crate of the current dir
    ///
    /// Checks it from Cargo.toml
    pub(crate) fn check_crate_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.directory() {
            Err(_) => {
                return Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "Cargo.toml not found")));
            }
            Ok(_) => {}
        }

        let manifest = Manifest::from_path("Cargo.toml")?;
        if let Some(package) = manifest.package {
            let name = package.name;

            Ok(name)
        } else {
            Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "Package Name")))
        }
    }

    /// Checks if package given is in target/doc
    pub(crate) fn check_package(&self, package: &str) -> Result<bool, Box<dyn std::error::Error>> {
        match self.directory() {
            Err(_) => {
                return Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "Cargo.toml cant be seen")));
            }
            Ok(_) => {}
        }

        if self.dir.join("target/doc").join(package).exists() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
