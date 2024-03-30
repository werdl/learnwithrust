# Lifetimes
In Rust, lifetimes can be a source of errors and confusion for many, but are a powerful tool for ensuring memory safety. Lifetimes are a way to ensure that references are valid for as long as they are used. They are a way to tell the compiler how long a reference is valid for, and are used to prevent dangling pointers.

Don't worry if you don't understand what we mean by "dangling pointers" or "memory safety" yet. We'll explain these concepts in more detail later on, and if you ever pick up a language like C or C++, you will probably come to understand the pain C-style devs go through that we can thankfully avoid thanks to the Rust compiler. For now though, let's focus on understanding lifetimes.

## What are lifetimes?
Lifetimes are a way to tell the Rust compiler how long a reference is valid for. They are a way to ensure that references are valid for as long as they are used. They are a way to prevent dangling pointers.

### What is a reference? What is a dangling pointer?
A reference is a way to refer to a value without taking ownership of it. This is useful when you want to pass a value to a function without giving up ownership of it. Here's an example:

```rust
fn main() {
    let x = 5; // x is a value
    let y = &x; // y is a reference to x
    println!("x: {}", x); // prints 5
    println!("y: {}", y); // prints 5
}
```

In this example, `x` is a value, and `y` is a reference to `x`. We can use `y` to refer to `x` without taking ownership of `x`. This is useful when you want to pass a value to a function without giving up ownership of it.


A dangling pointer is a pointer that points to a value that has been deallocated. This can happen when a reference outlives the value it refers to. Here's an example:

```rust
fn main() {
    let y;
    {
        let x = 5; // x is a value
        y = &x; // y is a reference to x
    }
    println!("y: {}", y); // error: `x` does not live long enough
}
```
```c
// If you were to write the same code in C, you would get no such error

// Don't worry if you don't understand the below code - it's just to show that C doesn't have the same safety features as Rust
#include <stdio.h>

int main() {
    int *y;
    {
        int x = 5; // x is a value
        y = &x; // y is a reference to x
    }
    printf("y: %d\n", *y); // undefined behavior - no such error
}
```

In these examples, `x` is a value, and `y` is a reference to `x`. However, `x` goes out of scope before `y`, so `y` becomes a dangling pointer. This is a problem because `y` is now pointing to a value that has been deallocated, which can lead to undefined behavior.

## How to use lifetimes
Now that we know what lifetimes are, let's see how to use them.

### Lifetime annotations
In Rust, lifetimes are denoted by single quotes (`'`). You can use lifetime annotations to tell the Rust compiler how long a reference is valid for. Here's an example:

```rust
fn main() {
    let x = 5; // x is a value
    let y: &'static i32 = &x; // y is a reference to x with the 'static lifetime
    println!("x: {}", x); // prints 5
    println!("y: {}", y); // prints 5
}

fn my_function<'a>(x: &'a i32) {
    println!("x: {}", x);
}

fn dealing_with_a_str(x: &'static str) {
    println!("x: {}", x);
}
```

## What is the 'static lifetime?
The `'static` lifetime is a special lifetime that means the reference is valid for the entire duration of the program. This is useful when you want to store a reference to a value that will never be deallocated. Here's an example:

```rust
fn main() {
    let x = 5; // x is a value
    let y: &'static i32 = &x; // y is a reference to x with the 'static lifetime
    println!("x: {}", x); // prints 5
    println!("y: {}", y); // prints 5
}
```

## What is the 'a lifetime? What is the 'b lifetime?
The `'a` and `'b` lifetimes are generic lifetimes that can be used to specify the lifetime of a reference. You can use them to tell the Rust compiler how long a reference is valid for. Here's an example:

```rust
fn main() {
    let x = 5; // x is a value
    let y: &'a i32 = &x; // y is a reference to x with the 'a lifetime
    println!("x: {}", x); // prints 5
    println!("y: {}", y); // prints 5
}

fn my_function<'a>(x: &'a i32) {
    println!("x: {}", x);
}

fn a_and_b<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x: {}", x);
    println!("y: {}", y);
}
```

In this example, `x` is a value, and `y` is a reference to `x` with the `'a` lifetime. You can use the `'a` and `'b` lifetimes to specify the lifetime of a reference.

## Conclusion
Lifetimes are a powerful tool for ensuring memory safety in Rust, but they can be avoided in many cases by using other features of the language. However, they are still an important concept to understand, as they are a key part of the Rust language. We hope this guide has helped you understand lifetimes better, and that you feel more confident using them in your own code.

If you are still unsure, try reading the official Rust book, which has a great section on lifetimes. You can also try experimenting with lifetimes in your own code to get a better feel for how they work. Good luck!

Still unsure? Just liberally sprinkle `'static` everywhere where `&str` gives you an error and you'll be fine. Just kidding - please don't do that. 😅