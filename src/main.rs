use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //aside: run with `cargo run -- "body" "./poem.txt" `
    // with environment var:  IGNORE_CASE=5 cargo run -- To poem.txt
    //or cargo test to run tests

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).
                     unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for `{}`", config.query);
    println!("In file {}\n", config.file_path);
    
   if let Err(e) =  minigrep::run(config) {
       eprintln!("Application error: {e}");
       process::exit(1);
   }
}
