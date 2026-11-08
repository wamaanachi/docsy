use std::io::Error;
use std::path::PathBuf;
use cargo_toml::Manifest;

/// Checks if the current directory is an active rust repo
///
/// Checks for a Cargo.toml, otherwise returns error
pub(crate) fn directory() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir: PathBuf = std::env::current_dir()?;

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
pub(crate) fn check_crate_name() -> Result<String, Box<dyn std::error::Error>> {
    match directory() {
        Err(_) => {
            return Err(Box::new(Error::new(std::io::ErrorKind::NotFound, "Cargo.toml")));
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
