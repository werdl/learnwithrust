# Hello, World!
With Rust installed, we can now write and run our first program.
## Writing the Program
Open your text editor, and create a new file called `main.rs`. This is the standard filename for a Rust program. In this file, write the following code:
```rust
fn main() {
    println!("Hello, world!");
}
```
## Running the Program
Open your terminal, and navigate to the directory where you saved `main.rs`, using `cd`. Then, run the following command:
```sh
rustc main.rs
```
This will compile your program, and create an executable file called `main` (or on Windows, `main.exe`). To run your program, use the following command:
```sh
./main # don't worry, this works on Windows too
```
You should see the following output:
```sh
Hello, world!
```
Congratulations! You've just written and run your first Rust program. You're now a programmer! 🎉
## I don't understand what I just did
```rust
fn // this is a function, a unit of code that can be called from elsewhere in the program
main // this one is called main (which is the one called by the computer)
() // it has no arguments
{
    println! // we call the println macro (a special kind of adaptable function)
    ("Hello, world!") // and pass it the string "Hello, world!" (a series of characters)
    ; // and end the line with a semicolon (which is how Rust knows that the line is over)
}
```