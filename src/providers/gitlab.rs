use crate::config::{GitSyncConfig, GitSyncConfigOrg};
use crate::providers::shared::{
    add_authorization_to_builder, add_to_path, get_git_ssh_url_segments, get_parent_folder,
};
use crate::repo::RepoInfo;
use log::debug;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

const DEFAULT_HOST: &str = "https://gitlab.com";
const DEFAULT_PAGINATION_SIZE: i32 = 100;

#[derive(Debug, Deserialize)]
struct GitlabProjectResponse {
    ssh_url_to_repo: String,
}

fn create_url(path: &String) -> Url {
    let mut url = Url::parse(DEFAULT_HOST).expect("Host could not be parsed");

    url.set_path(path.as_str());
    url.set_query(Some(
        format!(
            "include_subgroups=true&per_page={}",
            DEFAULT_PAGINATION_SIZE
        )
        .as_str(),
    ));

    debug!("URL: {:?}", url.as_str());

    url
}

async fn gitlab_generic_repos_handler(
    config: &GitSyncConfig,
    owner: &GitSyncConfigOrg,
    path: &String,
) -> Vec<RepoInfo> {
    let url = create_url(path);

    let mut req_builder = Client::new().get(url.as_str());

    req_builder = add_authorization_to_builder(req_builder, &owner.auth_token);

    let mut repos: Vec<GitlabProjectResponse> = vec![];
    let mut page = 1;

    loop {
        let req_builder_part = req_builder
            .try_clone()
            .expect("Request builder could not be cloned")
            .query(&[("page", page)]);

        debug!("URL: {:?}", &req_builder_part);

        let mut repos_segment = req_builder_part
            .send()
            .await
            .expect("Gitlab request failed")
            .json::<Vec<GitlabProjectResponse>>()
            .await
            .expect("Gitlab response could not be parsed");

        debug!("Repos segment: {:?}", repos_segment);

        let segment_length = repos_segment.len();

        repos.append(&mut repos_segment);

        if segment_length < DEFAULT_PAGINATION_SIZE as usize {
            debug!(
                "Breaking out of loop: length '{}', page size '{}'",
                segment_length, DEFAULT_PAGINATION_SIZE
            );
            break;
        }

        page = page + 1;
    }

    debug!("GitLab group projects {:?}", repos);

    repos
        .iter()
        .map(|r| {
            let (namespace, path) = get_git_ssh_url_segments(&r.ssh_url_to_repo);

            let local_repo_path = add_to_path(&config.path, &vec![namespace, path]);

            debug!("Local repo path {:?}", local_repo_path.to_str());

            let local_folder_path = get_parent_folder(&local_repo_path);

            debug!("Local folder path {:?}", local_folder_path.to_str());

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

    gitlab_generic_repos_handler(config, owner, &path).await
}

pub async fn gitlab_group_handler(
    config: &GitSyncConfig,
    owner: &GitSyncConfigOrg,
) -> Vec<RepoInfo> {
    let path = format!("/api/v4/groups/{}/projects", owner.namespace);

    gitlab_generic_repos_handler(config, owner, &path).await
}
