use log::{debug, info};
use std::path::PathBuf;
use tokio::process::Command;

pub async fn git_command(args: &Vec<&str>, cwd: &PathBuf) -> Result<(), String> {
    let output = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .await;

    if output.is_err() {
        let out = output.unwrap_err().to_string();
        return Err(format!("Git command failed: {out}"));
    }

    let out = output.unwrap();
    if !out.status.success() {
        return Err(format!("Git command failed: {:?}, {:?}, {:?}", args, cwd, out));
    }

    debug!("git {}: {:?}", args[0], String::from_utf8(out.stdout));

    Ok(())
}

pub async fn git_clone_mirror(git_url: &str, cwd: &PathBuf) -> Result<(), String> {
    info!("Cloning {:?} into {:?}", git_url, &cwd);
    let res = git_command(&vec!["clone", "--mirror", git_url], cwd).await;
    info!("Cloned {:?}", &git_url);
    res
}

pub async fn git_mirror_update(cwd: &PathBuf) -> Result<(), String> {
    info!("Updating {:?}", cwd);
    let res = git_command(&vec!["remote", "update"], cwd).await;
    info!("Updated {:?}", cwd);
    res
}
