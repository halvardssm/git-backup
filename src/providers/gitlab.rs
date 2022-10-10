use crate::config::{GitSyncConfig, GitSyncConfigOrg};
use crate::providers::shared::{
    add_to_path, folder_handler, get_git_ssh_url_segments, get_parent_folder,
};
use crate::repo::RepoInfo;
use log::debug;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

const DEFAULT_HOST: &str = "https://gitlab.com";

#[derive(Debug, Deserialize)]
struct GitlabProjectResponse {
    ssh_url_to_repo: String,
}

fn create_url(path: String) -> Url {
    let mut url = Url::parse(DEFAULT_HOST).expect("Host could not be parsed");

    url.set_path(path.as_str());

    debug!("URL: {:?}", url.as_str());

    url
}

fn github_generic_repos_handler(
    config: &GitSyncConfig,
    owner: &GitSyncConfigOrg,
    repos: Vec<GitlabProjectResponse>,
) -> Vec<RepoInfo> {
    repos
        .iter()
        .map(|r| {
            let (namespace, path) = get_git_ssh_url_segments(&r.ssh_url_to_repo);

            let local_repo_path =
                add_to_path(&config.path, &vec![owner.provider.clone(), namespace, path]);

            debug!("Local repo path {:?}", local_repo_path.to_str());

            let local_folder_path = get_parent_folder(&local_repo_path);

            debug!("Local folder path {:?}", local_folder_path.to_str());

            folder_handler(&local_folder_path);

            return RepoInfo {
                url: r.ssh_url_to_repo.clone(),
                local_folder_path: local_folder_path.clone(),
                local_repo_path,
            };
        })
        .collect::<Vec<RepoInfo>>()
}

pub async fn gitlab_user_handler(
    config: &GitSyncConfig,
    owner: &GitSyncConfigOrg,
) -> Vec<RepoInfo> {
    let path = format!("/api/v4/users/{}/projects", owner.namespace);

    let url = create_url(path);

    let repos = Client::new()
        .get(url.as_str())
        .send()
        .await
        .expect("Gitlab request failed")
        .json::<Vec<GitlabProjectResponse>>()
        .await
        .expect("Gitlab response could not be parsed");

    debug!("GitLab group projects {:?}", repos);

    github_generic_repos_handler(config, owner, repos)
}

pub async fn gitlab_group_handler(
    config: &GitSyncConfig,
    owner: &GitSyncConfigOrg,
) -> Vec<RepoInfo> {
    let path = format!("/api/v4/groups/{}/projects", owner.namespace);

    let url = create_url(path);

    let repos = Client::new()
        .get(url.as_str())
        .send()
        .await
        .expect("Gitlab request failed")
        .json::<Vec<GitlabProjectResponse>>()
        .await
        .expect("Gitlab response could not be parsed");

    debug!("GitLab group projects {:?}", repos);

    github_generic_repos_handler(config, owner, repos)
}
