use env_logger::Env;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process::Command;
use tokio::time::{self, Duration};

fn run_git_reset_branch(path: &str) {
    let output_fetch = Command::new("git")
        .current_dir(path)
        .arg("fetch")
        .arg("--prune")
        .output();
    let t = output_fetch.unwrap();
    if !t.status.success() {
        error!("git fetch: {:?}", t);
        return;
    }

    info!("git fetch: {:?}", String::from_utf8(t.stdout));

    let output_status = Command::new("git")
        .current_dir(path)
        .arg("status")
        .arg("--porcelain")
        .arg("-b")
        .output();
    let t = output_status.unwrap();
    if !t.status.success() {
        error!("git status: {:?}", t);
        return;
    }

    let status = String::from_utf8(t.stdout).unwrap();
    if !status.contains("[behind") && !status.contains("[ahead") {
        return;
    }

    let output_reset = Command::new("git")
        .current_dir(path)
        .arg("reset")
        .arg("--hard")
        .arg("origin")
        .output();
    let t = output_reset.unwrap();
    if !t.status.success() {
        error!("git reset: {:?}", t);
        return;
    }

    info!("git reset: {:?}", String::from_utf8(t.stdout));

    let output_clean = Command::new("git")
        .current_dir(path)
        .arg("clean")
        .arg("-df")
        .output();
    let t = output_clean.unwrap();
    if !t.status.success() {
        error!("git clean: {:?}", t);
        return;
    }

    info!("git clean: {:?}", String::from_utf8(t.stdout));
}

async fn run_interval(repo: RepoConfig) {
    let mut interval = time::interval(Duration::from_secs(repo.interval));
    let path = repo.path.as_str();
    loop {
        interval.tick().await;
        info!("Running {}", repo.path);
        run_git_reset_branch(&path);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GitSyncConfigRepo {
    path: String,
    interval: Option<u64>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GitSyncConfig {
    interval: u64,
    repos: Vec<GitSyncConfigRepo>,
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

async fn get_config() -> Result<GitSyncConfig, serde_yaml::Error> {
    let path: String = get_config_path();
    debug!("Config path: {}", path);
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    debug!(": {}", contents);

    let config: GitSyncConfig = serde_yaml::from_str(&contents).expect("Not valid config");

    Ok(config)
}

#[derive(Debug)]
struct RepoConfig {
    path: String,
    interval: u64,
}

fn parse_config(config: GitSyncConfig) -> Vec<RepoConfig> {
    let default_interval = config.interval;
    let mut vec: Vec<RepoConfig> = Vec::new();

    for repo in &config.repos {
        let r = RepoConfig {
            interval: repo.interval.unwrap_or(default_interval),
            path: repo.path.clone(),
        };

        vec.push(r)
    }

    vec
}

#[tokio::main]
async fn main() {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let config = get_config().await.unwrap();
    debug!("Config raw: {:?}", config);

    let parsed_config = parse_config(config);
    debug!("Repos parsed: {:?}", parsed_config);

    for repo in parsed_config {
        run_interval(repo).await
    }
}
