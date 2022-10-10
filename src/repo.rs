use crate::config::GitSyncConfig;
use crate::providers;
use log::{debug, info};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RepoInfo {
    pub url: String,
    pub local_folder_path: PathBuf,
    pub local_repo_path: PathBuf,
}

pub async fn get_repos(config: &GitSyncConfig) -> Vec<RepoInfo> {
    info!("get_repos:start");

    let mut repos: Vec<RepoInfo> = Vec::new();

    for repo in &config.repos {
        let r = providers::individual::repo_parser(config, repo).await;
        repos.push(r);
    }

    for org in &config.owners {
        match org.provider.as_str() {
            "github_user" => {
                let mut r = providers::github::github_user_handler(config, org).await;
                repos.append(&mut r);
            }
            "github_org" => {
                let mut r = providers::github::github_org_handler(config, org).await;
                repos.append(&mut r);
            }
            "gitlab_user" => {
                let mut r = providers::gitlab::gitlab_user_handler(config, org).await;
                repos.append(&mut r);
            }
            "gitlab_org" => {
                let mut r = providers::gitlab::gitlab_group_handler(config, org).await;
                repos.append(&mut r);
            }
            _ => println!("No provider available: {}", org.provider),
        }
    }

    debug!("Repos: {:?}", repos);
    info!("get_repos:end");

    repos
}
