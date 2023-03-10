use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
	    return Err("not enough arguments");
	}
        let query = args[1].clone(); // the program's name is &arg[0]
        let file_path = args[2].clone();
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case, 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // "?" will return error instead of `expect` which panics
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


pub fn search<'a>(query: &str, contents:  &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    /*
    * Iterate through each line of the contents.
    * Check whether the line contains our query string.
    * If it does, add it to the list of values we’re returning.
    * If it doesn’t, do nothing.
    * Return the list of results that match.
    */
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents:  &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod  tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";
        assert_eq!(vec!["        safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";
        assert_eq!(vec!["Rust:","        Trust me."], search_case_insensitive(query, contents));
    }
}