pub mod data;
pub mod hub;
use anyhow::Error;

pub async fn install(uvd: &str) -> Result<(), Error> {
    println!(">>> Installing {uvd}");
    Ok(())
}

pub async fn build() -> Result<(), Error> {
    println!(">>> Building");
    Ok(())
}
pub async fn reinstall(uvd: &str) -> Result<(), Error> {
    println!(">>> Reinstalling {uvd}");
    Ok(())
}

pub async fn uninstall(uvd: &str) -> Result<(), Error> {
    println!(">>> Uninstalling {uvd}");
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
    println!(">>> Adding dependencies {deps}");
    Ok(())
}
pub async fn remove_dependency(deps: &str) -> Result<(), Error> {
    println!(">>> Removing dependencies {deps}");
    Ok(())
}
pub async fn archive() -> Result<(), Error> {
    println!(">>> Archiving");
    Ok(())
}
pub async fn info(uvd: &str) -> Result<(), Error> {
    println!(">>> Getting info for {uvd}");
    Ok(())
}
pub async fn export(uvd: &str, usb: &str) -> Result<(), Error> {
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
