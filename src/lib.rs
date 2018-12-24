use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content =  fs::read_to_string(config.filename)?;
    println!("Full file text:\n{}", file_content);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }
        Ok (Self {
            query: args[1].clone(),
            filename: args[2].clone()
        })
    }
}