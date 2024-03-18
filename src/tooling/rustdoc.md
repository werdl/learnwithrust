# Rustdoc
Rustdoc is a tool that generates documentation for your code from comments in your code.
## Writing Documentation Comments
To write documentation comments, you use `///` for single-line comments and `/** */` for multi-line comments. For example:
```rust
/// This is a single-line documentation comment
fn add(x: i32, y: i32) -> i32 {
    x + y
}

/**
 * This is a multi-line documentation comment
 * It can span multiple lines
 */
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}
```
## Generating Documentation
To generate documentation from your code, you can use the `rustdoc` command, followed by the name of the file that you want to generate documentation for. Here's an example:
```sh
rustdoc main.rs
```
Alternatively, you can run
```sh
cargo doc
```
It will then give you a link to the documentation in your terminal, which you can open in your web browser.
## Viewing Documentation
To view the documentation for a crate, you can use the `cargo doc --open` command. This will generate the documentation for all the crates in your project, and open it in your web browser.
## Writing Documentation
When writing documentation, you can use Markdown to format your text. For example, you can use `#` for headings, `*` for bullet points, and `[]()` for links. Here's an example:
```rust
/// # Example
/// Here's an example of how to use the `add` function:
/// ```rust
/// let sum = add(5, 10);
/// println!("The sum of 5 and 10 is: {}", sum);
/// ```
/// # Links
/// You can also add links to other parts of the documentation, like this:
/// For more information, see the [add] function.
/// ```
/// [add]: fn.add.html
```
This will format the documentation like this:
---
# Example
Here's an example of how to use the `add` function:
```rust
let sum = add(5, 10);
println!("The sum of 5 and 10 is: {}", sum);
```
# Links
You can also add links to other parts of the documentation, like this:
For more information, see the [add] function.

[add]: fn.add.html
---