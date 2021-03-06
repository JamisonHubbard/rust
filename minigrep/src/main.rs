use std::env;
use std::process;

use lib::*;

fn main() {
    // parse args
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("problem parsing args: {}", err);
            process::exit(1);
        });

    // printing display
    println!("Looking for \"{}\"\nIn file {}\n", 
        config.query, 
        config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}