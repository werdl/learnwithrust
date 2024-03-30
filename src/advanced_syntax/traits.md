# Traits
In Rust, traits are a way to define behavior that types can implement.

## What does that mean?
When we define a trait, we are defining a set of methods that a type must implement in order to be considered an implementation of that trait. This allows us to define behavior that can be shared across different types.

Previously, we learned about structs and enums. We can define methods on structs and enums using `impl` blocks. Traits allow us to define methods that can be implemented by any type that implements the trait, rather than having to discipline yourself, not doing it correctly is a compile-time error.

## Defining a Trait
To define a trait, we use the `trait` keyword followed by the name of the trait and a set of method signatures. Here's an example of a simple trait:

```rust
trait Animal {
    fn speak(&self);
}
```

Alternatively, we can define default implementations for methods in a trait. This allows us to provide a default implementation that can be overridden by types that implement the trait. Here's an example of a trait with a default implementation:

```rust
trait Animal {
    fn speak(&self) { // Default implementation - does not have to be implemented by types that implement the trait, but can be overridden by defining them in the usual fashion
        println!("Some generic animal sound");
    }

    fn color(&self) -> String; // This method must be implemented by types that implement the trait
}
```

## Implementing a Trait
To implement a trait for a type, we use the `impl` keyword followed by the trait name. Here's an example of implementing the `Animal` trait for a `Dog` type:

```rust
struct Dog {
    name: String,
    color: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }

    fn color(&self) -> String {
        color
    }
}
```

Alternatively, we can implement a trait for a type outside of the type's definition. This is useful when we want to implement a trait for a type that we don't own. Here's an example of implementing the `Animal` trait for a `Cat` type:

```rust
use cat_library::Cat; // Assume that the Cat type is defined in the cat_library crate

impl Animal for Cat { // we can implement the Animal trait for the Cat type even though we don't own the Cat type, because we own the Animal trait
    fn speak(&self) {
        println!("Meow!");
    }

    fn color(&self) -> String {
        color
    }
}
```

## Using Traits
Once we have defined a trait and implemented it for a type, we can use the trait to call the methods defined in the trait. Here's an example of using the `Animal` trait to call the `speak` method on a `Dog` type:

```rust
let dog = Dog {
    name: String::from("Rover"),
    color: String::from("Brown"),
};

dog.speak(); // prints "Woof!"
println!("The dog's color is {}", dog.color()); // prints "The dog's color is Brown"
```

## Trait Bounds
Trait bounds are a way to specify that a generic type must implement a certain trait. This allows us to use methods defined in the trait on the generic type.

### What does that mean?
When we define a generic type, we can specify that the type must implement a certain trait by using a trait bound. This allows us to use methods defined in the trait on the generic type.

Some traits are defined in the standard library, such as `std::fmt::Debug`, which allows us to print a value using the `{:?}` format specifier. Here's an example of a generic function that uses a trait bound:

```rust
fn print_debug<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value);
}
```

This syntax might look a bit strange at first, but it's actually quite logical. The `T: std::fmt::Debug` syntax specifies that `T` must implement the `std::fmt::Debug` trait. This allows us to use the `{:?}` format specifier to print the value. 

This means that `T` can be any type that implements the `std::fmt::Debug` trait. This is a powerful feature of Rust's type system, as it allows us to write generic functions that work with any type that implements a certain trait.

Then, we use a feature (`{:?}`) that can be used with any type that implements the `std::fmt::Debug` trait.

### Multiple Trait Bounds
We can specify multiple trait bounds by using the `+` operator. This allows us to specify that a generic type must implement multiple traits. Here's an example of a generic function that uses multiple trait bounds:

```rust
fn print_debug_and_display<T: std::fmt::Debug + std::fmt::Display>(value: T) {
    println!("{:?}", value);
    println!("{}", value);
}
```

This syntax specifies that `T` must implement both the `std::fmt::Debug` and `std::fmt::Display` traits. This allows us to use both the `{:?}` and `{}` format specifiers to print the value.

Alternatively, we can use the `where` keyword to specify trait bounds. This can make the code easier to read, especially when specifying multiple trait bounds. Here's an example of using the `where` keyword to specify trait bounds:

```rust
fn print_debug_and_display<T>(value: T)
where
    T: std::fmt::Debug + std::fmt::Display,
{
    println!("{:?}", value);
    println!("{}", value);
}

fn print_two_different_things<T, U>(value1: T, value2: U)
where
    T: std::fmt::Debug,
    U: std::fmt::Display,
{
    println!("{:?}", value1);
    println!("{}", value2);
}
```

These two examples function similarly to the previous example, but they use the `where` keyword to specify the trait bounds. This can make the code easier to read, especially when specifying multiple trait bounds, so it's a good habit to get into.

## The `derive` Attribute and Some Commonly Used Traits
The `derive` attribute allows us to automatically implement certain traits for a type. This is useful for implementing traits that have a standard implementation that can be derived from the type's structure.

For example, the `Debug` trait allows us to print a value using the `{:?}` format specifier. We can automatically implement the `Debug` trait for a type by using the `derive` attribute. Here's an example of using the `derive` attribute to implement the `Debug` trait for a `Point` type:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let my_point = Point { x: 10, y: 20 };

println!("{:?}", my_point); // "Point { x: 10, y: 20 }"
```
### Commonly Used Traits
Here are some commonly used traits in Rust:
> Can be automatically derived using the `derive` attribute:
- `Debug`: Allows us to print a value using the `{:?}` format specifier.
- `Clone`: Allows us to create a copy of a value.
- `Copy`: Allows us to copy a value instead of moving it.
- `PartialEq`: Allows us to compare two values for equality.
- `PartialOrd`: Allows us to compare two values for ordering.
- `Default`: Allows us to create a default value for a type.
- `Hash`: Allows us to compute a hash value for a value.
- `Eq`: Allows us to compare two values for equality.
- `Ord`: Allows us to compare two values for ordering.

> Must be implemented manually:
- `Display`: Allows us to print a value using the `{}` format specifier.
- `From`: Allows us to convert a value from one type to another.
- `Into`: Allows us to convert a value from one type to another.
- `Iterator`: Allows us to iterate over a collection of values.

More details on these traits can be found in the Rust documentation - a far more authoritative source than this book!

#### `PartialEq` and `PartialOrd` vs `Eq` and `Ord`
The `PartialEq` and `PartialOrd` traits are used for types that can be compared for equality and ordering, respectively. These traits are used for types that don't have a total ordering, such as floating-point numbers.

The `Eq` and `Ord` traits are used for types that have a total ordering, such as integers. These traits are used for types that have a total ordering, such as integers.

Floating point numbers cannot be totally ordered because of the existence of `NaN` (Not a Number) values, which must be implemented as defined the `IEEE-754` spec (similar issues may arise for `-0` and `+0`). This is why we have `PartialEq` and `PartialOrd` for floating point numbers, but integers do not have such problems (there is no arguably strange value like `NaN` for integers, as there is no spec (at least one that I could find) that defines such a value for integers).