use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    
    // let (query, destination) = parse_config(&args);

    let query = &args[1];
    let destination = &args[2];

    println!("Searching for {}", config.query);
    println!("In file {}", config.destination);

    let contents = fs::read_to_string(destination)
        .expect("Should have been able to read the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    destination: String,
}

fn parse_config(args: &[String]) -> (Config) {
    let query = args[1].clone();
    let destination = args[2].clone();

    Config {query (), destination ()}
}