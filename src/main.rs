use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("Searching in file {}", config.filename);

    let file_content =  fs::read_to_string(config.filename)
        .expect("Could not read the file.");
    println!("Full file text:\n{}", file_content);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        Self {
            query: args[1].clone(),
            filename: args[2].clone()
        }
    }
}