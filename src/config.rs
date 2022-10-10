use log::debug;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::fs;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitSyncConfigRepo {
    pub url: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitSyncConfigOrg {
    pub provider: String,
    pub namespace: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitSyncConfig {
    #[serde(default = "get_default_interval")]
    pub interval: u64,
    #[serde(default = "get_default_path")]
    pub path: String,
    #[serde(default)]
    pub repos: Vec<GitSyncConfigRepo>,
    #[serde(default)]
    pub owners: Vec<GitSyncConfigOrg>,
}

fn get_default_interval() -> u64 {
    86400
}

fn get_default_path() -> String {
    String::from("./git-backup-repos")
}

fn get_config_path() -> String {
    for argument in env::args() {
        if argument.starts_with("--config=") {
            let v: Vec<&str> = argument.split("=").collect();
            return v[1].to_string();
        }
    }

    return "./git_sync_config.yaml".to_string();
}

pub async fn get_config() -> GitSyncConfig {
    let path: String = get_config_path();
    debug!("Config path: {}", path);

    let contents = fs::read_to_string(path)
        .await
        .expect("Failed to read config file");
    debug!("Config raw: {}", contents);

    let config: GitSyncConfig = serde_yaml::from_str(&contents).expect("Not valid config");
    debug!("Config parsed: {}", contents);

    config
}
