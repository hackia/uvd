use anyhow::Error;
/// add a remote url
/// # Errors
/// - if remote already exists
pub fn add_remote(name: &str, url: &str) -> Result<(), Error> {
    println!(">>> Adding remote {name} with url {url}");
    Ok(())
}
/// remove a remote url
/// # Errors
/// - if remote not found
pub fn remove_remote(name: &str) -> Result<(), Error> {
    println!(">>> Removing remote {name}");
    Ok(())
}
