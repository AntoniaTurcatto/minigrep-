use std::{env, error::Error, fs, process};
use minigrep::{search, search_case_insensitive};

fn main() {    
    let config = Config::build(env::args()).unwrap_or_else(|err|{//it returns the inner value that Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure
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

    let results: Vec<&str> = if config.ignore_case{
        search_case_insensitive(&config.query, &contents).collect()
    } else {
        search(&config.query, &contents).collect()
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
    /*The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args, 
    and that type implements the Iterator trait and returns String values. */

    /*Because we’re taking ownership of args and we’ll be mutating args by iterating over it, 
    we can add the mut keyword into the specification of the args parameter to make it mutable.*/
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str>{
        args.next();
        let  query = args.next().ok_or_else(|| "Didn't get a query string")?;
        println!("query: {query}");

        let file_path = args.next().ok_or_else(|| "Didn't get a file path")?;
        println!("file_path: {file_path}");

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, file_path, ignore_case })
    }
}