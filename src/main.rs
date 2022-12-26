use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    //dbg!(args); // print for debugging, removed later
    
    let query = &args[1]; // the program's name is &arg[0]
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
