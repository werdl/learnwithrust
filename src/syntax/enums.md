# Enums
Enums are a kind of data type with a fixed set of possible values.

## Defining an enum
You can define an enum using the `enum` keyword followed by the name of the enum and then a list of possible values inside curly braces. Here's an example of an enum that represents a `Direction`:
```rust
enum Direction { // Define an enum called `Direction`
    Up, // Define a value called `Up`
    Down, // Define a value called `Down`
    Left, // Define a value called `Left`
    Right, // Define a value called `Right`
}
```

In this example, we define an enum called `Direction` with four possible values: `Up`, `Down`, `Left`, and `Right`.

## Using an enum
You can use an enum by creating an instance of the enum and then matching on its value. Here's an example of using the `Direction` enum:
```rust
let direction = Direction::Up; // Create an instance of the `Direction` enum

match direction { // Match on the value of `direction`
    Direction::Up => println!("Moving up!"), // Match on the `Up` value
    Direction::Down => println!("Moving down!"), // Match on the `Down` value
    Direction::Left => println!("Moving left!"), // Match on the `Left` value
    Direction::Right => println!("Moving right!"), // Match on the `Right` value
}
```

In this example, we create an instance of the `Direction` enum called `direction` and then use a `match` expression to match on its value. Depending on the value of `direction`, we print a different message.

## Associated data
Enums can also have associated data. This means that each value of the enum can have additional data associated with it. Here's an example of an enum that represents a `Message`:
```rust
enum Message { // Define an enum called `Message`
    Quit, // Define a value called `Quit`
    Move { x: i32, y: i32 }, // Define a value called `Move` with associated data
    Write(String), // Define a value called `Write` with associated data
    ChangeColor(i32, i32, i32), // Define a value called `ChangeColor` with associated data
}
```

In this example, we define an enum called `Message` with four possible values. The `Move` value has associated data in the form of an anonymous struct, and the `Write` and `ChangeColor` values have associated data in the form of a `String` and three `i32` values, respectively.

## Using associated data
You can use associated data by creating an instance of the enum value and then matching on its associated data. Here's an example of using the `Message` enum:
```rust
let message = Message::Move { x: 10, y: 20 }; // Create an instance of the `Message` enum with associated data
let message_str = Message::Write(String::from("Hello, world!")); // Create an instance of the `Message` enum with associated data
let message_color = Message::ChangeColor(255, 0, 0); // Create an instance of the `Message` enum with associated data
let message_quit = Message::Quit; // Create an instance of the `Message` enum without associated data
```
## Functions in enums
You can define functions inside an enum using the `impl` keyword, much like structs. These functions are called methods. Here's an example of defining a method for the `Message` enum:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to ({}, {}, {})", r, g, b),
        }
    }
}
```
Let's break down this example:
- We define an enum called `Message` with four possible values. The `Move` value has associated data in the form of an anonymous struct, and the `Write` and `ChangeColor` values have associated data in the form of a `String` and three `i32` values, respectively.
- We use the `impl` keyword to define a block of methods for the `Message` enum.
- Inside the `impl` block, we define a method called `call` that takes a reference to `self` as a parameter.
- Inside the `call` method, we use a `match` expression to match on the value of `self` and then print a message based on the value.
- We can then call the `call` method on instances of the `Message` enum.
