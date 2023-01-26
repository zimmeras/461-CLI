use std::env;
use cli::rate_repos as lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    match &input[..] {
        "install" => install(),
        "build" => build(),
        "tes" => test(),
        url_file_path => lib::parse_url_file(url_file_path),
    };
}

fn install() {
    println!("Installing dependencies");
}

fn build() {}
fn test() {}
