use std::env;
use std::fs;
use std::process;
use std::error::Error;

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
    
   // not using unwrap here since this is for error handling only
   // unwrap is useful if we plan to do something with returned object
   if let Err(e) =  run(config) {
       println!("Application error: {e}");
       process::exit(1);
   }
}

// extract this part from main as run
// use Result to avoid panic due to 'expext' from earlier version
// can return a dynamic error train
// if all is well, then we return (), i.e. not really returning a type.. just side effects

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // "?" will return error instead of `expect` which panics
    let contents = fs::read_to_string(config.file_path)?;

    println!("With test: \n{contents}");

    Ok(())
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