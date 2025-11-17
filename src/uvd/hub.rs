use anyhow::Error;

///
/// login to the hub
///
/// # Errors
/// - if not possible
pub fn login() -> Result<(), Error> {
    println!(">>> Login");
    Ok(())
}
/// logout from the hub
/// # Errors
/// - if not possible
pub fn logout() -> Result<(), Error> {
    println!(">>> Logout");
    Ok(())
}
