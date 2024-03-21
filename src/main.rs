use std::{env, process};
use grsp::{Config, run};

fn main() {
    // 读取参数
    let args:Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("run error: {e}");
        process::exit(1);
    }
}