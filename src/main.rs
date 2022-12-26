use std::env;
use std::fs;

fn main() {
    //aside: run with cargo run -- "the" "./poem.txt"

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    
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
    fn new(args: &[String]) -> Config {
        if args.len() < 3{
	    panic!{"not enough arguments"};
	}
        let query = args[1].clone(); // the program's name is &arg[0]
        let file_path = args[2].clone();
        
        Config {query, file_path }
    }
}