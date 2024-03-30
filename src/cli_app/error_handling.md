# Error Handling
Now, we will implement the core functionality of our CLI app: reading the contents of a file and printing them to the terminal. We will also handle the case where the file does not exist.

## Reading a File
First, we need to write the code to read the contents of a file. We will use the `std::fs` module (a library written by Rust) to read the file, and we will use the `std::io` module to handle any errors that occur.

Open the `src/main.rs` file and add the following code:

```rust
use std::fs;
use std::io::Read;
use std::io;

fn read_from_file(file: &str) -> Result<String, io::Error> {
    let mut contents = String::new();

    let file_result = fs::File::open(file);

    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let read_result = file.read_to_string(&mut contents);

    match read_result {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    } // Return the result of the match expression
}
```

Let's break down what's happening in this code:
- First, we bring in the `fs` and `io` modules from the standard library using the `use` keyword.
- We define a function called `read_from_file` that takes a reference to a `&str` as a parameter and returns a `Result<String, io::Error>`. This means that the function can return either a `String` or an `io::Error`.
- Inside the `read_from_file` function, we use the `fs::read_to_string` function to read the contents of the file.

## Handling Errors
Rust takes an innovative approach to error handling. Instead of using exceptions like many other languages, Rust uses the `Result` type to represent the possibility of an error. This forces the programmer to explicitly handle errors, which can help prevent bugs and make the code more reliable.
## `Result<T>` Type
The `Result` type is an enum that has two variants: `Ok` and `Err`. The `Ok` variant represents a successful result, and the `Err` variant represents an error. The `Result` type is generic, which means it can hold any type of value for the `Ok` variant and any type of error for the `Err` variant.

Here's an example of using the `Result` type to handle errors:

```rust
use std::fs;
use std::io::Read;

fn main() {
    let mut contents = String::new(); // Create a mutable variable to hold the file contents
    let result = fs::File::open("hello.txt").unwrap().read_to_string(&mut contents); // Try to open the file and read its contents
    match result {
        Ok(_) => println!("File contents: {}", contents), // If the result is Ok, print the file contents
        Err(e) => eprintln!("Error reading file: {}", e), // If the result is Err, print the error message
    }
}
```

In this example, we use the `fs::File::open` function to open the file, and then we use the `read_to_string` method to read its contents. We use the `unwrap` method to convert the `Result` into a `T` or panic if the result is `Err`. We then use a `match` expression to handle the `Result`. If the result is `Ok`, we print the file contents, and if the result is `Err`, we print the error message.

## `Option<T>` Type
The `Option` type is similar to the `Result` type, but it is used to represent the possibility of a value being absent. The `Option` type is also an enum that has two variants: `Some` and `None`. The `Some` variant represents a value, and the `None` variant represents the absence of a value.

Here's an example of using the `Option` type to handle the possibility of a value being absent:

```rust
fn main() {
    let x: Option<i32> = Some(5); // Create an `Option` that contains a value
    let y: Option<i32> = None; // Create an `Option` that does not contain a value
    match x {
        Some(v) => println!("Value: {}", v), // If `x` contains a value, print the value
        None => println!("No value"), // If `x` does not contain a value, print a message
    }
    match y {
        Some(v) => println!("Value: {}", v), // If `y` contains a value, print the value
        None => println!("No value"), // If `y` does not contain a value, print a message
    }
}
```

### The `?` Operator
The `?` operator is a shorthand for propagating errors. It can be used in functions that return a `Result` or an `Option` to automatically return the error or `None` if the result is `Err` or `None`.

This way, rather than littering your code with `match` expressions, you can use the `?` operator to handle errors in a more concise way.

## Our Code
Let's go back to our `read_from_file` function and see how we can use the `?` operator to handle errors more concisely:

```rust
use std::fs;
use std::io::Read;
use std::io;

fn read_from_file(file: &str) -> Result<String, io::Error> {
    let mut contents = String::new();

    fs::File::open(file)?.read_to_string(&mut contents)?;

    Ok(contents)
}
```
Now, instead of using `match` expressions to handle the `Result` of each operation, we use the `?` operator to automatically return the error if the result is `Err`.
## A little bit of homework
- Now we have a function that reads the contents of a file and returns a `Result`. How can we use this function in our CLI app to read the file name from the command line and print its contents to the terminal?
- What happens if the file does not exist? How can we handle this case?
- How can we improve the error messages that are printed to the terminal?
- Try doing this - if you get stuck, try having a look at the example code in the `models/cli_app` directory of the GitHub repository - (found here)[https://github.com/werdl/learnwithrust]