use std::env::args; 
use std::process;

use mini_grep_cli::{Config, run};

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::new(args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    }) ;

    println!("Searching for {} \nIn file {} ...", config.query, config.filepath);

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        process::exit(1);
    };
}


