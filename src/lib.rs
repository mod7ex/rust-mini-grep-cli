use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;

    let result;

    if config.case_sensitive {
        result = search(&config.query, &content);
    } else {
        result = search_case_insensitive(&config.query, &content);
    }

    for (index, line) in result {
        println!("[{}] {}", index, line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() != 3 {
            println!("[Usage] {} <$query> <$file_path>", &args[0]);
            return Err("Failed parsing arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filepath, case_sensitive })
    }
}

fn search<'a>(query: &'a str, content: &'a str) -> Vec<(u32, &'a str)> {
    let mut result = Vec::new();

    let mut i = 1;

    for line in content.lines() {
        if line.contains(query) {
            result.push((i, line));
        }

        i += 1;
    }

    result
}

fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<(u32, &'a str)> {
    let mut result = Vec::new();

    let query = query.to_lowercase();

    let mut i = 1;

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push((i, line));
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duck";
        let contents = "\
Rust:
safe, safe, production.
Pick three.
Duck";
    
        assert_eq!(
            vec![(2, "safe, safe, production.")], 
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "tHrEe";
        let contents = "\
Rust:
safe, safe, production.
Pick Three.";
    
        assert_eq!(
            vec![(3, "Pick Three.")], 
            search_case_insensitive(query, contents)
        );
    }
}