use std::{env, process};

use cli::Config;

fn main() {
    // Get arguments
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    // let file_path = &args[2];
    // let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Read file content
    if let Err(e) = cli::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
