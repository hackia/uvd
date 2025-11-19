use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub os: HashMap<String, Vec<PackageInfo>>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageInfo {
    #[serde(rename = "type")]
    pub package_type: String,
    pub arch: String,
    pub distro_target: Option<String>,
    pub url: String,
    pub hash_sha256: String,
}
