pub mod data;
use anyhow::Error;

pub async fn install(uvd: &str) -> Result<(), Error> {
    println!(">>> Installing {uvd}");
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

