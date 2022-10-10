use crate::config::{GitSyncConfig, GitSyncConfigOrg};
use crate::providers::shared::{folder_handler, get_parent_folder, add_to_path};
use crate::repo::RepoInfo;
use log::debug;
use octocrab;

fn github_generic_repos_handler(config: &GitSyncConfig, owner: &GitSyncConfigOrg, repos: Vec<octocrab::models::Repository>) -> Vec<RepoInfo> {
    repos
        .iter()
        .map(|r| {
            let local_repo_path = add_to_path(
                &config.path,
                &vec![
                    owner.provider.clone(),
                    r.clone_url.clone().expect("Github user SSH url was not available").path().to_string().trim_start_matches("/").to_string(),
                ],
            );

            debug!("Local repo path {:?}",local_repo_path.to_str());

            let local_folder_path = get_parent_folder(&local_repo_path);

            debug!("Local folder path {:?}",local_folder_path.to_str());

            folder_handler(&local_folder_path);

            return RepoInfo {
                url: r.ssh_url.clone().unwrap().to_string(),
                local_folder_path: local_folder_path.clone(),
                local_repo_path,
            };
        })
        .collect::<Vec<RepoInfo>>()
}

pub async fn github_user_handler(config: &GitSyncConfig, owner: &GitSyncConfigOrg) -> Vec<RepoInfo> {
    let url = format!("/users/{}/repos", owner.namespace);
    let repos: Vec<octocrab::models::Repository> = octocrab::instance()
        .get(url.clone(), None::<&()>)
        .await
        .expect(format!("Github call failed for {}", url).as_str());

    debug!("Github repos '{}' {:?}", url, repos);

    github_generic_repos_handler(config,owner,repos)
}

pub async fn github_org_handler(config: &GitSyncConfig, owner: &GitSyncConfigOrg) -> Vec<RepoInfo> {
    let url = format!("/orgs/{}/repos", owner.namespace);
    let repos: Vec<octocrab::models::Repository> = octocrab::instance()
            .get(url.clone(), None::<&()>)
            .await
            .expect(format!("Github call failed for {}", url).as_str());

    debug!("Github repos '{}' {:?}", url, repos);

    github_generic_repos_handler(config,owner,repos)
}
