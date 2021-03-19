use mygrep::Config;
use std::env;
use std::process;

#[cfg(test)] mod tests;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Parsing arguments failed: {}", err);
        process::exit(1);
    });

    if let Err(err) = mygrep::run(config) {
        eprintln!("Processing failed: {}", err);
        process::exit(1);
    }
}
