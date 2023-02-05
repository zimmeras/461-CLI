use std::env;
use purdue461_cli::rate_repos;
use purdue461_cli::rate_repos::metrics::correctness;


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let input = &args[1];
    match &input[..] {
        "install" => install(),
        "build" => build(),
        "tes" => test(),
        url_file_path => rate_repos::rate_repos(url_file_path),
    };
    //correctness::clone_repo("https://github.com/cloudinary/cloudinary_npm");
    correctness::delete_repo();
}

fn install() {
    println!("Installing dependencies");
}

fn build() {}
fn test() {}
