use std::fs;
use std::error::Error;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content =  fs::read_to_string(config.filename)?;
    for line in search(&config.query, &file_content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let args = vec![String::from("a"), String::from("query"), String::from("file")];
        let config = Config::new(&args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "file");
    }

    #[test]
    fn search_content() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(search(query, content), vec!("safe, fast, productive."));
    }
}