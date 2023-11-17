use std::fs;
use std::error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search_case_sensitive(&config.query, &content)
    };
    for line in results {
        println!("{line}");
    }
    return Ok(());
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next(); // skip the program name
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config { query, file_path, ignore_case });
    }
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    return content.lines().filter(|line| line.contains(query)).collect();
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    return content.lines()
        .filter(|line| line.to_lowercase().contains(&query)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
