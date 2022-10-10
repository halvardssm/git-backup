use log::{debug, info};
use std::path::PathBuf;
use tokio::process::Command;

pub async fn git_command(args: &Vec<&str>, cwd: &PathBuf) {
    let output = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .await
        .expect("git command failed");

    if !output.status.success() {
        panic!("git; args {:?}; cwd {:?}; output {:?}", args, cwd, output);
    }

    debug!("git {}: {:?}", args[0], String::from_utf8(output.stdout));
}

pub async fn git_clone_mirror(git_url: &str, cwd: &PathBuf) {
    info!("Cloning {:?} into {:?}", git_url, &cwd);
    git_command(&vec!["clone", "--mirror", git_url], cwd).await;
    info!("Cloned {:?}", &git_url)
}

pub async fn git_mirror_update(cwd: &PathBuf) {
    info!("Updating {:?}", cwd);
    git_command(&vec!["remote", "update"], cwd).await;
    info!("Updated {:?}", cwd);
}
