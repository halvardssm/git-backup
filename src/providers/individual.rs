use regex::Regex;
use crate::config::{GitSyncConfig, GitSyncConfigRepo};
use crate::providers::shared::{folder_handler, get_parent_folder, add_to_path, get_git_ssh_url_segments};
use crate::repo::RepoInfo;

pub async fn repo_parser(config: &GitSyncConfig, repo: &GitSyncConfigRepo) -> RepoInfo {
    let re = Regex::new(r".+@.+:\w.+").expect("Not valid regex");

    if !re.is_match(repo.url.as_str()) {
        panic!("Url is not in SSH format, was {:?}", repo.url);
    }

    let (namespace, path) =get_git_ssh_url_segments(&repo.url);

    let local_repo_path = add_to_path(
        &config.path,
        &vec![
            String::from("individual"),
            namespace,
            path
        ]);

    let local_folder_path = get_parent_folder(&local_repo_path);

    folder_handler(&local_folder_path);

    let url = repo.url.clone();

    RepoInfo {
        local_folder_path,
        local_repo_path,
        url,
    }
}
