use std::env;
use std::fs;

fn main() {
    //aside: run with cargo run -- "the" "./poem.txt"

    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With test: \n{contents}");
}


fn parse_config(args: &[String]) -> (&str, &str){
    let query = &args[1]; // the program's name is &arg[0]
    let file_path = &args[2];
    
    (query, file_path)    
}