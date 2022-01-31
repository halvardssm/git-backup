use env_logger::Env;
use log::{debug, error, info};
use octocrab::params;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
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
struct GitSyncConfigOrg {
    provider: String,
    namespace: String,
    path: String,
    interval: Option<u64>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GitSyncConfig {
    default_interval: u64,
    #[serde(default)]
    repos: Vec<GitSyncConfigRepo>,
    #[serde(default)]
    owners: Vec<GitSyncConfigOrg>,
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
    let default_interval = config.default_interval;

    let mut repos: Vec<RepoConfig> = Vec::new();

    for repo in &config.repos {
        let r = RepoConfig {
            interval: repo.interval.unwrap_or(default_interval),
            path: repo.path.clone(),
        };

        repos.push(r)
    }

    repos
}

fn git_command(args: &Vec<&str>, path: Option<&PathBuf>) {
    let mut c = Command::new("git");
    if path.is_some() {
        c.current_dir(path.unwrap());
    }
    let output = c.args(args).output();
    let res = output.unwrap();
    if !res.status.success() {
        error!("git {}: {:?}", args[0], res);
        return;
    }

    info!("git fetch: {:?}", String::from_utf8(res.stdout));
}

fn git_clone_mirror(git_url: &str, path: &PathBuf) {
    git_command(&vec!["clone", "--mirror", git_url], Some(path));
}

fn git_mirror_update(path: &PathBuf) {
    git_command(&vec!["remote", "update"], Some(path));
}

struct RepoInfo {
    name: String,
    url: String,
    local_folder_path: PathBuf,
    local_repo_path: PathBuf,
    interval: u64,
}

async fn backup_handler(repos: Vec<RepoInfo>) {
    for repo in repos {
        let repo_url = repo.url;
        if !repo.local_repo_path.exists() {
            info!("Cloning {:?}", &repo.local_repo_path);
            git_clone_mirror(repo_url.as_str(), &repo.local_repo_path);
        }

        let mut interval = time::interval(Duration::from_secs(repo.interval));
        loop {
            interval.tick().await;
            info!("Running {:?}", repo.local_repo_path);
            git_mirror_update(&repo.local_repo_path);
        }
    }
}

fn folder_handler(path_raw: String) -> PathBuf {
    let path = PathBuf::from(path_raw.as_str());
    if !path.exists() {
        let res = fs::create_dir_all(path.clone());
        if res.is_err() {
            panic!("Path could not be created {:?}", res.err())
        }
    } else if !path.is_dir() {
        panic!("Provided path {:?} is not a directory", path)
    }

    return path;
}

async fn github_user_handler(default_interval: &u64, owner: GitSyncConfigOrg) -> Vec<RepoInfo> {
    let url = format!("/users/{}/repos", owner.namespace);
    let repos: Result<Vec<octocrab::models::Repository>, octocrab::Error> =
        octocrab::instance().get(url, None::<&()>).await;
    let repos = repos.unwrap();

    let local_folder_path = folder_handler(owner.path);
    repos
        .iter()
        .map(|r| {
            let name = r.name.clone();
            let mut local_repo_path = local_folder_path.join(&name);
            local_repo_path.set_extension("git");

            return RepoInfo {
                name: name,
                url: r.git_url.clone().unwrap().to_string(),
                interval: owner.interval.unwrap_or(*default_interval),
                local_folder_path: local_folder_path.clone(),
                local_repo_path: local_repo_path,
            };
        })
        .collect::<Vec<RepoInfo>>()
}

#[tokio::main]
async fn main() {
    let env = Env::default().default_filter_or("trace");
    env_logger::init_from_env(env);

    let config = get_config().await.unwrap();
    debug!("Config raw: {:?}", config);

    let default_interval = config.default_interval;

    let mut repos: Vec<RepoInfo> = Vec::new();

    for org in config.owners {
        match org.provider.as_str() {
            "github_user" => {
                let mut r = github_user_handler(&default_interval, org).await;
                repos.append(&mut r);
            }
            _ => println!("No provider available: {}", org.provider),
        }
    }

    backup_handler(repos).await;
}
