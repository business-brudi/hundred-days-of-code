use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let configuration = parse_configuration(&args);
    println!("searching for '{}'", configuration.query);
    println!("in file '{}'", configuration.file_path);

    let contents = fs::read_to_string(configuration.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Configuration {
    query: String,
    file_path: String
}

fn parse_configuration(args: &[String]) -> Configuration {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Configuration { query, file_path }
}
