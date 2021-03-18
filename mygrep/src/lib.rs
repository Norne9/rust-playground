use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let search_fn = if config.case_sensitive {
        search
    } else {
        search_ignore_case
    };

    for result in search_fn(config.query, &content) {
        println!("{}", result);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[derive(Debug, PartialEq)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        let query = args.get(1).ok_or("Query is not given!")?;
        let filename = args.get(2).ok_or("Filename is not given!")?;

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config { query, filename, case_sensitive })
    }
}
