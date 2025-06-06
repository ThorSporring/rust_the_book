
use std::fs;
use std::error::Error;

const VALID_NUMBER_OF_ARGUMENTS: usize = 3;

pub struct Config{
    query: String,
    file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < VALID_NUMBER_OF_ARGUMENTS{
            return Err("not enough arguments")
        }
        
        Ok(
            Config{
            query: args[1].clone(),
            file_path: args[2].clone()
        }
    )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("Lines matching case: ");
    println!("_____________________");
    for line in search(&config.query, &contents){
        println!("{line}");
    }

    println!("Lines matching case insensitive: ");
    println!("_____________________");
    for line in search_case_insensitive(&config.query, &contents){
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let mut q = query.clone();
    
    for line in contents.lines(){
        if line.to_lowercase().contains(&q.to_lowercase()){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
