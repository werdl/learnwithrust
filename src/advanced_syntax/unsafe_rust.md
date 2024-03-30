# Unsafe Rust
Though much of this book is focused on safe Rust, there are cases when you need to opt out of some of Rust's guarantees and write unsafe code. Unsafe code is a set of operations that the Rust compiler cannot guarantee are safe, and it's up to the programmer to ensure they are used correctly. Unsafe code is used when you need to:
- Dereference a raw pointer
- Call an unsafe function or method (for example, a C function, or a function interacting with the operating system or hardware)
- Access or modify a mutable static variable (static variables are variables with the `'static` lifetime, defined in the global scope)
- Implement an unsafe trait
- Access fields of `union`s
- Mutate a static variable

## What is unsafe code?
Unsafe code is a set of operations that the Rust compiler cannot guarantee are safe. This means that the Rust compiler cannot ensure that the code follows the rules of safe Rust. It's up to the programmer to ensure that the code is used correctly.

Here's an example of unsafe code:
```rust
fn main() {
    let x = 5;
    let y = &x as *const i32; // create a raw pointer to x
    let z = unsafe { *y }; // dereference the raw pointer
    println!("z: {}", z); // prints 5
}
```

In this example, we create a raw pointer `y` to the value `x`, and then dereference the raw pointer to get the value `z`. This code is unsafe because the Rust compiler cannot guarantee that the raw pointer `y` is valid.

## When to use unsafe code
You should only use unsafe code when you have to. Unsafe code is a powerful tool, but it comes with risks. If you use unsafe code incorrectly, you can introduce bugs, security vulnerabilities, or undefined behavior into your program. Here are some common cases when you might need to use unsafe code:
- Interfacing with C code
- Implementing low-level abstractions
- Writing performance-critical code
- Accessing or modifying static variables

If you do not understand why you need to use unsafe code, you should avoid it. It's better to write safe code whenever possible.

If you do not understand what you are writing, you should avoid writing unsafe code. If you do need to write unsafe code, you should document why you need to use it and what invariants you are relying on.

## How to write unsafe code
To write unsafe code in Rust, you need to use the `unsafe` keyword. The `unsafe` keyword tells the Rust compiler that the code inside the block is unsafe and that the programmer is responsible for ensuring its safety. Here's an example:
```rust
fn main() {
    let x = 5;
    let y = &x as *const i32; // create a raw pointer to x
    let z = unsafe { *y }; // dereference the raw pointer
    println!("z: {}", z); // prints 5
}
```

Key points to remember when writing unsafe code:
- Use `unsafe` only when necessary
- Document why you need to use unsafe code
- Ensure that the unsafe code is correct and safe to use
- Test the unsafe code thoroughly
- Use safe abstractions whenever possible
- Avoid writing unsafe code if you don't understand it
- Where possible, use trusted libraries that provide safe abstractions rather than writing unsafe code yourself