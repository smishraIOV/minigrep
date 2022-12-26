use std::env;
use std::fs;
use std::process;

fn main() {
    //aside: run with cargo run -- "the" "./poem.txt"

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).
                     unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With test: \n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // "new" is now "build" and has a new signature using Result<>
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
	    return Err("not enough arguments");
	}
        let query = args[1].clone(); // the program's name is &arg[0]
        let file_path = args[2].clone();
        
        Ok(Config {query, file_path })
    }
}