use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            Err(String::from(
                "Not enough arguments, enter search string and file name",
            ))
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{line}");
        }
    } else {
        for line in search(&config.query, &content) {
            println!("{line}");
        }
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn searches_for_duct_in_content() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
";
        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
