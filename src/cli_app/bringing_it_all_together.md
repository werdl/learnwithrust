# Bringing It All Together
Now we have each component of our CLI app in place. We have the ability to parse command line arguments, handle errors, write tests, and document our code. Let's bring it all together and see how it works.
## The Final Code
Here's the final code for our CLI app:
```rust
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
```

Yours might look a bit different - but that's okay! The important thing is that you understand how each part works and how they fit together.

## Running the App
Now that we have written the code for our CLI app, we can run it to see if it works. Run the following command to build and run the app:
```sh
cargo run -- Cargo.toml
```

You should see the following output:
```sh
Reading file: Cargo.toml
File contents:
[package]
name = "cli_app"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
clap = { version = "4.5.3", features = ["derive"] }
```
Again, yours might look a bit different, depending on the contents of your `Cargo.toml` file. But you should see the contents of the file printed to the terminal. You can check that the output is correct by opening the file in a text editor and comparing the contents.

## Conclusion
Building a CLI app is a great way to learn Rust. We have covered a lot of ground in this chapter, including parsing command line arguments, error handling, testing, and documentation. We have also used the `clap` crate to make our app more user-friendly.