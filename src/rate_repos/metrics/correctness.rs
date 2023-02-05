use git2::Repository;
use std::fs;

//git url here
pub fn clone_repo(url: &str) {
     let _file = match fs::create_dir_all("./repos/new") {
        Ok(_file) => _file,
        Err(e) => panic!("failed to create temporary directory: {}", e),
    };
    let _repo = match Repository::clone(url, "./repos/new") {
        Ok(_repo) => _repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

pub fn delete_repo() {
    let _file = match fs::remove_dir_all("./repos/new") {
        Ok(_file) => _file,
        Err(e) => panic!("failed to remove temporary directory: {}", e),
    };
}