use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    //dbg!(args); // print for debugging, removed later
    
    let query = &args[1]; // the program's name is &arg[0]
    let file_path = &args[2];

    //run with cargo run -- "the" "./poem.txt"
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With test: \n{contents}");
}
