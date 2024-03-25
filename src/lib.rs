use std::{env, error::Error, fs, io};
use regex::{Regex, RegexSetBuilder};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // 构造函数
    pub fn new() -> Config {
        let query = String::new();
        let file_path = String::new();
        let ignore_case = false;
        Config{query, file_path, ignore_case}
    }

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
        contents = fs::read_to_string(&config.file_path)?;
    }

    let query_result = search(&config, &contents);
    for line in query_result{
        println!("{}", colored(line, &config.query));
    }
    Ok(())
}

pub fn colored<'a>(line: &'a str, query: &'a str) -> String {
    let re = Regex::new(query).unwrap();
    let colored_s = re.replace_all(&line, "\x1B[31m$0\x1B[0m");
    colored_s.to_string()
}

pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str>{
    let mut res = Vec::new();
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query.to_string()
    };
    let set = RegexSetBuilder::new(&[query.as_str()]).case_insensitive(config.ignore_case).build().unwrap();

    for line in contents.lines(){
        if set.is_match(line){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let mut config = Config::new();
        config.query = "duct".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(&config, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let mut config = Config::new();
        config.query = "rUsT".to_string();
        config.ignore_case = true;
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search(&config, contents)
        );
    }
}
