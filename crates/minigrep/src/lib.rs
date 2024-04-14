use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    #[allow(clippy::missing_errors_doc)]
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let Some(query) = args.next() else {
            return Err("Didn't get a query string");
        };

        let Some(file_path) = args.next() else {
            return Err("Didn't get a file path");
        };

        let ignore_case = should_ignore_case(&mut args);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[must_use]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[must_use]
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[must_use]
fn should_ignore_case(args: &mut impl Iterator<Item = String>) -> bool {
    if let Some(arg) = args.next() {
        let lowercased_arg = arg.to_lowercase();
        if lowercased_arg == "--case_insensitive" || lowercased_arg == "-ci" {
            return true;
        }
    }

    env::var("IGNORE_CASE").is_ok()
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
