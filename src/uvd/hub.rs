use crate::uvd::data::Manifest;
use anyhow::{Context, Result};
use inquire::{Password, Text};
use reqwest::Client;
const HUB_URL: &str = "https://hub.hackia.org/api/v1";

#[derive(Clone, Default)]
pub struct UvdClient {
    client: Client,
    api_url: String,
}

impl UvdClient {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_url: HUB_URL.to_string(),
        }
    }
    ///
    /// search a package on the universal verified disc hub
    ///
    /// # Errors
    /// api failure
    /// bad user input
    pub async fn search_package(&self, query: &str) -> Result<Vec<Manifest>> {
        let url = format!("{}/search?q={}", self.api_url, query);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("failed to send request to Hub")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("hub error: {}", response.status()));
        }
        let results: Vec<Manifest> = response.json().await?;
        Ok(results)
    }
    /// login to the universal verified disc hub
    /// # Errors
    /// on api failure
    pub async fn login(&self, username: &str, token: &str) -> Result<()> {
        let url = format!("{}/auth/login", self.api_url);

        let payload = serde_json::json!({
            "username": username,
            "token": token
        });

        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            println!(">>> Logged as {username}");
            Ok(())
        } else {
            Err(anyhow::anyhow!("hub error: {}", response.status()))
        }
    }
}

/// Connect to the universal verified disc hub
/// # Errors
/// on bad user input
/// on api failure
pub async fn login() -> Result<()> {
    println!(">>> Connexion au Hub...");
    let client = UvdClient::new();
    let mut username = String::new();
    let mut token = String::new();
    while username.is_empty() {
        username.clear();
        username.push_str(Text::new("Username").prompt()?.as_str());
    }
    while token.is_empty() {
        token.clear();
        token.push_str(Password::new("Token").prompt()?.as_str());
    }
    client.login(username.as_str(), token.as_str()).await
}
