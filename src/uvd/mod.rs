pub mod data;
use anyhow::Error;

pub async fn install(uvd: &str) -> Result<(), Error> {
    println!(">>> Installing {uvd}");
    Ok(())
}
