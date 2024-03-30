# Structs
In Rust, a struct is a custom data type that you can use to create your own data structures. You can define a struct using the `struct` keyword and then define the fields of the struct inside curly braces.
## What is a Struct?
- A struct is a custom data type that you can use to create your own data structures.
- It groups related data together.
## Defining a struct
You can define a struct using the `struct` keyword and then define the fields of the struct inside curly braces. Here's an example of a struct that represents a `Person`:
```rust
struct Person { // Define a struct called `Person`
    name: String, // Define a field called `name` of type `String`
    age: u8, // Define a field called `age` of type `u8`
}
```

In this example, we define a struct called `Person` with two fields: `name` of type `String` and `age` of type `u8`.
## Creating an Instance of a Struct
You can create an instance of a struct by using the struct's name and then providing values for each field. Here's an example of creating an instance of the `Person` struct:
```rust
let person = Person { // Create an instance of the `Person` struct
    name: String::from("Alice"), // Set the `name` field to "Alice"
    age: 30, // Set the `age` field to 30
};
```
## Accessing Fields
You can access the fields of a struct using dot notation. Here's an example of accessing the `name` field of the `person` instance:
```rust
println!("Name: {}", person.name); // Access the `name` field of the `person` instance
```
## Functions in Structs
You can define functions inside a struct using the `impl` keyword. These functions are called methods. Here's an example of defining a method for the `Person` struct:
```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) { // Define a method called `say_hello`
        println!("Hello, my name is {}!", self.name); // Access the `name` field using `self`
    }
}
```
Let's break down this example:
- We define a struct called `Person` with two fields: `name` of type `String` and `age` of type `u8`.
- We use the `impl` keyword to define a block of methods for the `Person` struct.
- Inside the `impl` block, we define a method called `say_hello` that takes a reference to `self` as a parameter.
- Inside the `say_hello` method, we use the `self` keyword to access the `name` field of the `Person` instance.
## Associated Functions
You can define functions inside a struct that do not take a reference to `self`. These functions are called associated functions. Here's an example of defining an associated function for the `Person` struct:
```rust
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Person { // Define an associated function called `new`
        Person { // Create and return a new instance of the `Person` struct
            name,
            age,
        }
    }
}
```
This is quite similar to defining a regular function, but we use the `impl` keyword to define the function inside the context of the `Person` struct. We can then call the associated function like this:
```rust
let person = Person::new(String::from("Bob"), 25); // Call the `new` associated function
```

In this example, we call the `new` associated function on the `Person` struct to create a new instance of the `Person` struct.