use std::{env, error::Error, fs};
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        // 跳过程序名
        args.next();
        // 错误处理
        // if args.len() < 3{
        //     return Err("not enough args")
        // }
        let query = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a query string"),
        };
        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case})
    }
}

pub fn run(config:Config)-> Result<(), Box<dyn Error>>{
    // 读取文件
    let contents = fs::read_to_string(config.file_path)?;
    let query_result = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in query_result{
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents.lines().filter(|line| line.contains(query)).collect()
    // let mut res = Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         res.push(line);
    //     }
    // }
    // res
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