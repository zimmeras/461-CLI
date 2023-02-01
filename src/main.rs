use std::env;
use purdue461_cli::rate_repos;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let input = &args[1];
    match &input[..] {
        "install" => install(),
        "build" => build(),
        "tes" => test(),
        url_file_path => rate_repos::rate_repos(url_file_path),
    };
}

fn install() {
    println!("Installing dependencies");
}

fn build() {}
fn test() {}
