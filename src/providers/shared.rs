use std::fs;
use std::path::PathBuf;

pub fn folder_handler(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path.clone()).expect("Failed to create dir");
    } else if !path.is_dir() {
        panic!("Provided path {:?} is not a directory", path)
    }
}

pub fn add_to_path(base_path:&String, path_segments: &Vec<String>) ->PathBuf{
    let mut local_repo_path = PathBuf::from(base_path);

    for path_segment in path_segments {
        local_repo_path.push(path_segment);
    }

    local_repo_path
}

pub fn get_parent_folder(path:&PathBuf)->PathBuf{
    path.parent().expect("Could not get parent folder").to_path_buf()
}

pub fn get_git_ssh_url_segments(url:&String) -> (String, String) {
    let url_base = url.split("@").last().expect("Url could not be split at @");
    let mut path_segments = url_base.split(":").collect::<Vec<&str>>();
    let path = path_segments.pop().expect("Url could not get path").to_string();
    let namespace = path_segments.pop().expect("Url could not get namespace").to_string();

    return (namespace,path)
}
