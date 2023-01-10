use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //aside: run with `cargo run -- "the" "./poem.txt" `
    //or cargo test to run tests

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).
                     unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for `{}`", config.query);
    println!("In file {}\n", config.file_path);
    
   if let Err(e) =  minigrep::run(config) {
       println!("Application error: {e}");
       process::exit(1);
   }
}
