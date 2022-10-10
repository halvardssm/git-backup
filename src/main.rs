extern crate core;

use crate::config::get_config;
use crate::git::{git_clone_mirror, git_mirror_update};
use crate::providers::shared::folder_handler;
use crate::repo::get_repos;
use env_logger::{init_from_env, Env};
use futures::future::join_all;
use log::{debug, info};
use tokio::time::{sleep, Duration};

mod config;
mod git;
mod providers;
mod repo;

#[tokio::main]
async fn main() {
    let env = Env::default().default_filter_or("info");
    init_from_env(env);

    let config = get_config().await;
    debug!("Config raw: {:?}", config);

    loop {
        info!("Starting backup sequence");

        let repos = get_repos(&config).await;

        let tasks = repos.iter().map(|repo| async {
            let repo = repo.clone();
            let repo_url = repo.url;
            if !repo.local_repo_path.exists() {
                folder_handler(&repo.local_folder_path);
                git_clone_mirror(repo_url.as_str(), &repo.local_folder_path).await;
            } else {
                git_mirror_update(&repo.local_repo_path).await;
            }
        });

        join_all(tasks).await;

        info!(
            "Done with backup sequence, pausing for {} seconds",
            config.interval
        );

        sleep(Duration::from_secs(config.interval)).await
    }
}
