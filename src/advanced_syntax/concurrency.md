# Concurrency
Concurrency - the ability to run multiple tasks at the same time - is a powerful feature of modern programming languages. In Rust, just as in any other languages, there are many equally valid approaches. We will focus on just one - the `std::thread` module.

## Threads
Threads are a way to run multiple tasks concurrently.

### What is a thread?
A thread is a lightweight process that can run concurrently with other threads. Threads share the same memory space, but have their own stack and registers. This allows them to run independently of each other.

#### Process? Memory space? Stack? Registers?
- A process is an instance of a program that is running on a computer.
- Memory space is the area of memory that a process can access.
- A stack is a data structure that stores information about function calls.
- Registers are small areas of memory that store data that is being used by the CPU (the "brain" of the computer).

### Creating a thread
You can create a new thread by calling the `std::thread::spawn` function and passing it a closure (think of it as a little mini-function) that contains the code you want to run in the new thread. Here's an example:

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| { // define a closure, which takes no arguments, which can be shown as there is nothing between the pipes (|)
        println!("Hello from a thread!");
    });

    handle.join().unwrap(); // wait for the thread to finish (this is not necessary, but it's a good practice to do so, to avoid the thread being killed before it finishes running (only needed at the end of a function, but if it is permissable, is good practice to do so))
}
```

In this example, we create a new thread that prints "Hello from a thread!".

### What is a handle?
A handle is a value that represents a thread. You can use it to wait for the thread to finish, or to send messages to the thread.

### What is a closure?
A closure is a way to define a block of code that can be passed around and executed later. In this example, we define a closure that prints "Hello from a thread!".

A closure can alternatively be defined as a function that captures its environment. This means that it can access variables from the surrounding scope.

#### The `move` keyword
By default, closures capture variables by reference. If you want to move ownership of a variable into a closure, you can use the `move` keyword. Here's an example:

```rust
use std::thread;

fn main() {
    let x = 5;

    let handle = thread::spawn(move || { // use the move keyword to move ownership of x into the closure
        println!("x: {}", x);
    });

    handle.join().unwrap(); // wait for the thread to finish
}
```