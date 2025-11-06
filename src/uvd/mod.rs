use std::env::consts::OS;
use std::fs::File;
use std::io::Write;
use std::{env, fs};
pub mod data;
pub mod hub;

use crate::license::get_licenses;
use crate::utils::run_hooks;
use anyhow::Error;
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs::create_dir_all;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub dependencies: Vec<String>,
}

pub fn install(uvd_path: &str) -> Result<(), Error> {
    let fichier_uvd = File::open(uvd_path)?;
    let decompresseur = zstd::stream::read::Decoder::new(fichier_uvd)?;
    let mut archive_tar = tar::Archive::new(decompresseur);

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
            return Err(Error::msg(format!("OS non supporté: {OS}")));
        }
    };
    fs::create_dir_all(&install_dir)?;
    archive_tar.unpack(&install_dir)?;
    println!(">>> Installed successfully in: {}", install_dir.display());
    Ok(())
}

pub fn reinstall(uvd: &str) -> Result<(), Error> {
    println!(">>> Reinstalling {uvd}");
    uninstall(uvd)?;
    install(uvd)?;
    println!(">>> Reinstalled {uvd}");
    Ok(())
}
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
            return Err(Error::msg(format!("OS non supporté: {OS}")));
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

pub async fn search(uvd: &str) -> Result<(), Error> {
    println!(">>> Searching {uvd}");
    Ok(())
}
pub async fn update(uvd: &str) -> Result<(), Error> {
    println!(">>> Updating {uvd}");
    Ok(())
}
pub async fn adding_dependency(deps: &str) -> Result<(), Error> {
    println!(">>> Adding dependency {deps}");
    Ok(())
}
pub async fn remove_dependency(deps: &str) -> Result<(), Error> {
    println!(">>> Removing dependency {deps}");
    Ok(())
}

pub fn uvd() -> Result<String, Error> {
    let config: Config = toml::from_str("uvd.toml")?;
    Ok(format!("{}_{}.uvd", config.name, config.version))
}
pub fn create_uvd() -> Result<(), Error> {
    if run_hooks().is_err() {
        return Err(Error::msg("Hooks failed"));
    };
    println!(">>> Creating");
    let archive_name = uvd()?;
    let archive_file = File::create(archive_name.clone())?;
    let compresseur = zstd::stream::write::Encoder::new(archive_file, 3)?;
    let mut archive_tar = tar::Builder::new(compresseur);
    archive_tar.append_path("Cargo.toml")?;
    archive_tar.append_path("uvd.toml")?;
    // ... ajoute d'autres fichiers (README, LICENSE...)

    // 5. Termine l'écriture
    // On doit appeler finish() sur le tar, puis sur le compresseur
    let compresseur_termine = archive_tar.into_inner()?;
    compresseur_termine.finish()?;

    println!("Archive {} créée !", archive_name.clone());
    Ok(())
}
pub async fn info(uvd: &str) -> Result<(), Error> {
    println!(">>> Getting info for {uvd}");
    Ok(())
}
pub async fn create_usb(uvd: &str, usb: &str) -> Result<(), Error> {
    println!(">>> Exporting {uvd} to {usb}");
    Ok(())
}
pub async fn verify(uvd: &str) -> Result<(), Error> {
    println!(">>> Verifying {uvd}");
    Ok(())
}
pub async fn publish() -> Result<(), Error> {
    println!(">>> Publishing");
    Ok(())
}
pub async fn upgrade() -> Result<(), Error> {
    println!(">>> Upgrading");
    Ok(())
}
pub async fn list() -> Result<(), Error> {
    println!(">>> Listing");
    Ok(())
}
pub async fn new() -> Result<(), Error> {
    let mut project = String::new();
    let mut license = String::new();

    while project.is_empty() {
        project.clear();
        project = Text::new("Project name").prompt()?;
    }
    if Path::new(&project).exists() {
        println!(">>> Project already exists");
        return Ok(());
    }

    create_dir_all(&project)
        .await
        .expect("Failed to create project directory");
    while license.is_empty() {
        license.clear();
        license = Select::new("Select a project license", get_licenses()).prompt()?;
    }
    let mut config = File::create(format!("{project}/uvd.toml"))?;
    let mut readme = File::create(format!("{project}/README.md"))?;
    let mut license_file = File::create(format!("{project}/LICENSE"))?;
    write!(config, "name = \"{}\"\nlicense = \"{}\"", project, license)?;
    write!(readme, "# {project}\n")?;
    write!(
        license_file,
        "This project is licensed under the {license} license.\n"
    )?;
    println!(">>> {project} initialized");
    Ok(())
}
