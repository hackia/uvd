use std::env::consts::OS;
use std::fs::{File, create_dir_all, read_to_string};
use std::io::Write;
use std::{env, fs};
pub mod data;
pub mod hub;

use crate::license::get_licenses;
use anyhow::Error;
use chrono::Utc;
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub readme: String,
    pub src: Vec<String>,
}

/// install an uvd package
/// # Errors
/// - If the package cannot be installed
pub fn install(uvd_path: &str) -> Result<(), Error> {
    let uvd_file = File::open(uvd_path)?;
    let dec = zstd::stream::read::Decoder::new(uvd_file)?;
    let mut archive_tar = tar::Archive::new(dec);

    let install_dir = match OS {
        "linux" | "freebsd" | "macos" => {
            let home = env::var("HOME")?;
            PathBuf::from(home).join(".uvd/bin")
        }
        "windows" => {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(appdata).join("uvd/bin")
        }
        _ => {
            println!(">>> OS not supported: {OS}");
            return Err(Error::msg(format!("OS not supported : {OS}")));
        }
    };
    create_dir_all(&install_dir)?;
    archive_tar.unpack(&install_dir)?;
    println!(">>> Installed successfully in: {}", install_dir.display());
    Ok(())
}

/// reinstall an uvd package
/// # Errors
/// - If the package cannot be reinstalled
pub fn reinstall(uvd: &str) -> Result<(), Error> {
    println!(">>> Reinstalling {uvd}");
    uninstall(uvd)?;
    install(uvd)?;
    println!(">>> Reinstalled {uvd}");
    Ok(())
}
/// uninstall an uvd package
/// # Errors
/// - If the package cannot be uninstalled
pub fn uninstall(uvd: &str) -> Result<(), Error> {
    let install_dir = match OS {
        "linux" | "freebsd" | "macos" => {
            let home = env::var("HOME")?;
            PathBuf::from(home).join(".uvd/bin")
        }
        "windows" => {
            let appdata = env::var("APPDATA")?;
            PathBuf::from(appdata).join("uvd/bin")
        }
        _ => {
            return Err(Error::msg(format!("OS non supported: {OS}")));
        }
    };

    let mut file_path = install_dir.join(uvd);

    if OS == "windows" && file_path.extension().is_none() {
        file_path.set_extension("exe");
    }

    if file_path.exists() {
        fs::remove_file(&file_path)?;
        println!(">>> '{uvd}' removed successfully.");
    } else {
        println!(">>> Package '{uvd}' not found (or already uninstalled).");
    }
    Ok(())
}
/// search for an uvd package
/// # Errors
/// - If the package cannot be searched
pub fn search(uvd: &str) -> Result<(), Error> {
    println!(">>> Searching {uvd}");
    Ok(())
}
/// update an uvd package
/// # Errors
/// - If the package cannot be updated
pub fn update(uvd: &str) -> Result<(), Error> {
    println!(">>> Updating {uvd}");
    Ok(())
}
/// add dependencies to an uvd package
/// # Errors
/// - If the dependencies cannot be added
pub fn adding_dependency(deps: &str) -> Result<(), Error> {
    println!(">>> Adding dependency {deps}");
    Ok(())
}
/// remove dependencies to an uvd package
/// # Errors
/// - If the dependencies cannot be removed
/// - If the dependencies cannot be removed
pub fn remove_dependency(deps: &str) -> Result<(), Error> {
    println!(">>> Removing dependency {deps}");
    Ok(())
}

/// # Errors
/// - if the config cannot be read
pub fn uvd() -> Result<String, Error> {
    let config: Config = toml::from_str(read_to_string("uvd.toml")?.as_str())?;
    Ok(format!(
        "{}-{}_{}.uvd",
        Utc::now().timestamp(),
        config.name,
        config.version
    ))
}
/// # Errors
/// - if the config cannot be read
/// - if the archive cannot be created
pub fn create_uvd() -> Result<(), Error> {
    let config: Config = toml::from_str(read_to_string("uvd.toml")?.as_str())?;
    println!(">>> Creating uvd from source code");
    let archive_name = uvd()?;
    let archive_file = File::create(archive_name.clone())?;
    let compressor = zstd::stream::write::Encoder::new(archive_file, 3)?;
    let mut archive = tar::Builder::new(compressor);

    for source in &config.src {
        if Path::new(source).is_dir() {
            archive.append_dir_all(source, source)?;
        } else {
            archive.append_path(source)?;
        }
    }
    let ending = archive.into_inner()?;
    ending.finish()?;
    println!("Archive {archive_name} has been created successfully",);
    Ok(())
}
/// # get info for an uvd package
/// # Errors
/// - if the info cannot be read
pub fn info(uvd: &str) -> Result<(), Error> {
    println!(">>> Getting info for {uvd}");
    Ok(())
}
/// create an usb drive from an uvd package
/// # Errors
/// - if the usb cannot be created
pub fn create_usb(uvd: &str, usb: &str) -> Result<(), Error> {
    println!(">>> Exporting {uvd} to {usb}");
    Ok(())
}
/// # verify an uvd package
/// # Errors
/// - if the package cannot be verified
pub fn verify(uvd: &str) -> Result<(), Error> {
    println!(">>> Verifying {uvd}");
    Ok(())
}
/// # Publish an uvd package
/// # Errors
/// - if the package cannot be published
pub fn publish() -> Result<(), Error> {
    println!(">>> Publishing");
    Ok(())
}
/// # Upgrade all universal verified discs
/// # Errors
/// - if the package cannot be upgraded
pub fn upgrade() -> Result<(), Error> {
    println!(">>> Upgrading");
    Ok(())
}
///
/// # Errors
/// - if not possible
pub fn list() -> Result<(), Error> {
    println!(">>> Listing");
    Ok(())
}
///
/// # create a new project
/// # Panics
/// - If the project directory cannot be created
/// # Errors
/// - If the project already exists
/// - If the project directory cannot be created
///
pub fn new() -> Result<(), Error> {
    let mut project = String::new();
    let mut license = String::new();

    while project.is_empty() {
        project.clear();
        project = Text::new("Project name:").prompt()?;
    }
    if Path::new(&project).exists() {
        println!(">>> Project already exists");
        return Ok(());
    }
    create_dir_all(&project).expect("Failed to create project directory");
    while license.is_empty() {
        license.clear();
        license = Select::new("Select a project license", get_licenses()).prompt()?;
    }
    let mut config = File::create(format!("{project}/uvd.toml"))?;
    let mut readme = File::create(format!("{project}/README.md"))?;
    let mut license_file = File::create(format!("{project}/LICENSE"))?;
    write!(config, "name = \"{project}\"\nlicense = \"{license}\"")?;
    writeln!(readme, "# {project}")?;
    writeln!(
        license_file,
        "This project is licensed under the {license} license."
    )?;
    println!(">>> {project} initialized");
    Ok(())
}
