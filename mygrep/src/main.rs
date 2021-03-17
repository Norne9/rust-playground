use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1).expect("Query is not given!");
    let filename = args.get(2).expect("Filename is not given!");
    println!("query: {} | file: {}", query, filename);

    let content = fs::read_to_string(filename)
        .expect(&format!("Can't read the file {}", filename));
    println!("Content: {}", content);
    
}
