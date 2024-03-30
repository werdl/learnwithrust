# Documentation
In this chapter, we will cover how to write documentation for your Rust code. We will use the `rustdoc` tool to generate documentation from our code, and we will also cover some best practices for writing good documentation.
## Remember doc comments?
We defined doc comments as comments that start with `///` or `/**`. These comments are used to document the purpose and usage of your code. They are parsed by the `rustdoc` tool to generate documentation for your code.
## Writing Documentation
To write documentation for your code, you can use doc comments to describe the purpose and usage of your code. You can also use markdown to format your documentation, including headings, lists, code blocks, and links.
Here's an example of a doc comment for a function:
```rust
/// Adds two numbers together.
/// Example:
/// ```rust
/// assert_eq!(add(2, 2), 4);
/// assert_eq!(add(1, 2), 3);
/// assert_eq!(add(-1, 1), 0);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
Now, when we document the code like previously detailed, we can generate and view docs for our crate.
## Another little bit of homework
- Write some documentation for the `read_from_file` function, and the `Cli` struct in our app.