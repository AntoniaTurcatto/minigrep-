use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{//it returns the inner value that Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });     

    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{//the function will return () or a type that implements the Error trait
    let contents = fs::read_to_string(config.file_path)?;        

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }

    Ok(())
}

struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config{
    fn build(args: &[String]) -> Result<Self, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}