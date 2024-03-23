use std::{env, error::Error, fs, io};
use regex::Regex;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // 跳过程序名
        args.next();
        let query = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string"),
        };
        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>String::new(),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case})
    }
}

pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    // 读取文件
    let contents ;
    if config.file_path == String::new() {
        contents = io::read_to_string(io::stdin()).unwrap();
    } else {
        contents = fs::read_to_string(config.file_path)?;
    }

    let query_result = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in query_result{
        println!("{}", colored(line, &config.query));
    }
    Ok(())
}

pub fn colored<'a>(line: &'a str, query: &'a str) -> String {
    let re = Regex::new(query).unwrap();
    let colored_s = re.replace_all(line, "\x1B[31m$0\x1B[0m");
    colored_s.to_string()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    // let mut res = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         res.push(line);
    //     }
    // }
    // res
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)-> Vec<&'a str>{
    // let mut res = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         res.push(line)
    //     }
    // }
    // res
    contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
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