use std::fs;
use std::io::Read;
use std::io;

use clap::Parser;


/// Read the contents of a file
///
/// ```rust
/// let contents = read_from_file("Cargo.toml").unwrap();
/// assert!(contents.contains("[package]"));
/// ```
fn read_from_file(file: &str) -> Result<String, io::Error> {
    let mut contents = String::new();

    fs::File::open(file)?.read_to_string(&mut contents)?;

    Ok(contents)
}


/// Command line arguments
#[derive(Parser)]
struct Cli {
    /// The file to read
    file: String,
}

fn main() {
    let args = Cli::parse();
    println!("Reading file: {}", args.file);

    match read_from_file(&args.file) {
        Ok(contents) => println!("File contents: {}\n", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}