use std::env;
use std::fs;

fn main() {
    //aside: run with cargo run -- "the" "./poem.txt"

    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    
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

fn parse_config(args: &[String]) -> Config {
    // using clone copies stuff.. so performance penalty. Fix with experience
    // a working program is higher priority now than optimization
    let query = args[1].clone(); // the program's name is &arg[0]
    let file_path = args[2].clone();
    
    Config {query, file_path }    
}