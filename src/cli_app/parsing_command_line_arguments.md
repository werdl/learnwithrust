# Parsing Command Line Arguments
In this section, we will set up the basic structure of our CLI app and use the `clap` crate to parse command line arguments.
## Setting Up the Project
First, we need to create a new Cargo project. Run the following command to create a new project called `minicat`:
```sh
cargo new minicat
```
Next, add `clap` as a dependency in the `Cargo.toml` file:
```sh
cargo add clap --features derive
```
Now, we can start writing the code for our CLI app. Open the `src/main.rs` file and add the following code:
```rust
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The file to read
    file: String,
}

fn main() {
    let args = Cli::parse();
    println!("Reading file: {}", args.file);
}
```
Let's break down what's happening in this code:
- We use the `clap` crate to define a struct called `Cli` that represents the command line arguments for our app. We use the `#[derive(Parser)]` attribute to automatically generate the parsing code for the struct.
### The `#[derive(Parser)]` Attribute
- derive means that our library will automatically implement some functionality for us. 
- This one implements the `Parser` set of functions for us, which allows us to parse command line arguments into our struct.
---
- The `Cli` struct has a single field called `file` of type `String`. This field represents the file name that we want to read.
- In the `main` function, we call the `parse` method on the `Cli` struct to parse the command line arguments. This method returns an instance of the `Cli` struct with the parsed arguments.
- We then print the file name to the terminal.

## Running the App
Now that we have written the code for our CLI app, we can run it to see if it works. Run the following command to build and run the app:
```sh
cargo run -- example.txt
```
You should see the following output:
```sh
Reading file: example.txt
```
Great! Our app is working as expected. We are now able to parse command line arguments using the `clap` crate. Now we will cover error handling in the next section - (Error Handling)[error_handling.md].