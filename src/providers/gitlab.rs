

async fn gitlab_handler(default_interval: u64, owner: &GitSyncConfigOrg) -> Vec<RepoInfo> {
    let url = format!("/users/{}/repos", owner.namespace);
    let repos: Result<Vec<octocrab::models::Repository>, octocrab::Error> =
        octocrab::instance().get(url, None::<&()>).await;
    let repos = repos.unwrap();

    let local_folder_path = folder_handler(&owner.path);
    repos
        .iter()
        .map(|r| {
            let name = r.name.clone();
            let mut local_repo_path = local_folder_path.join(&name);
            local_repo_path.set_extension("git");

            return RepoInfo {
                name: name,
                url: r.git_url.clone().unwrap().to_string(),
                interval: owner.interval.unwrap_or(default_interval),
                local_folder_path: local_folder_path.clone(),
                local_repo_path: local_repo_path,
            };
        })
        .collect::<Vec<RepoInfo>>()
}