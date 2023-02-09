use std::env;
use purdue461_cli::rate_repos;

fn main() -> Result<(), String> {
    let log_file = env::var("LOG_FILE").unwrap();
    let level = env::var("LOG_LEVEL");
    let log_level = match &level {
        Ok(t) => &t,
        Err(_e) => "0", //default level = 0
    };
    let level = match log_level{
        "0" => "trace",
        "1" => "info",
        "2" => "debug",
        _ => "error"
    };

    let config = simple_log::LogConfigBuilder::builder()
        .path(log_file)
        .level(level)
        .output_file()
        .build();
    simple_log::new(config)?;
    simple_log::info!("Sucessfully created log file");

    let args: Vec<String> = env::args().collect();
    
    let input = &args[1];
    match &input[..] {
        "install" => install(),
        "build" => build(),
        "tes" => test(),
        url_file_path => rate_repos::rate_repos(url_file_path),
    };
    Ok(())
}

fn install() {
    simple_log::info!("Installing dependencies");
}

fn build() {}
fn test() {}
