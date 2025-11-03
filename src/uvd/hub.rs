use anyhow::Error;

pub async fn login() -> Result<(), Error> {
    println!(">>> Login");
    Ok(())
}
pub async fn logout() -> Result<(), Error> {
    println!(">>> Logout");
    Ok(())
}
