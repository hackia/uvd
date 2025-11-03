use anyhow::Error;
pub async fn add_remote(name: &str, url: &str) -> Result<(), Error> {
    println!(">>> Adding remote {name} with url {url}");
    Ok(())
}
pub async fn remove_remote(name: &str) -> Result<(), Error> {
    println!(">>> Removing remote {name}");
    Ok(())
}
