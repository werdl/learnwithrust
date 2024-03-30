# Testing our CLI App
In this section, we will cover testing our CLI app. We will write unit tests for our functions and integration tests for our CLI app.
## Writing Unit Tests
A unit test is a test that verifies the correctness of a small piece of code, such as a function or a method. In Rust, we can write unit tests by creating a module with the `#[cfg(test)]` attribute and writing functions that test our code.

Here's an example of a unit test for a simple function:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 1), 0);
    }
}
```

Now, when we run `cargo test`, Rust will compile our code and run our tests. If all the tests pass, we will see a message indicating that all the tests passed. If any of the tests fail, Rust will print an error message with details about the failure.

## Writing some tests for our CLI app
Let's write some unit tests for our CLI app. We will write tests for the `read_from_file` function and the `main` function.

First, let's write a test for the `read_from_file` function:

```rust
use super::*;

#[test]
fn test_read_from_file() {
    let result = read_from_file("hello.txt");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello, world!");
}
```
Now, if `hello.txt` contains the string "Hello, world!", this test will pass. If the file does not exist or contains a different string, the test will fail.

## A little bit of homework
- Write some more unit tests for the `read_from_file` function to cover different cases, such as an empty file, a file with a large amount of data, and a file that does not exist.