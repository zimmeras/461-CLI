use std::env;
use purdue461_cli::rate_repos;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let url_file_path = &args[1];
    rate_repos::rate_repos(url_file_path);
}
