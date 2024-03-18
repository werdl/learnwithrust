# Variables
Variables are a named RAM location that holds a value, with one of the data types that we learned about in the [previous chapter](data_types.md). They are used to store data that can change while the program is running.
## Declaring Variables
- To declare a variable, we use the `let` keyword, followed by the name of the variable, and then a colon `:` and the data type of the variable.
- We can also add an equals sign `=` and the value that we want to store in the variable.
- Here's an example:
```rust
let x: i32 = 5;
```
- This declares a variable called `x` that holds a 32-bit signed integer with the value `5`.
- We can also declare a variable without giving it a value, like this:
```rust
let y: i32;
```
- But if we try to use `y` without giving it a value, the program will not compile - we must assign a value to `y` before we use it.
## Assigning Variables and Mutability
### Mutability?
- mutability: the ability to change a variable's value
- In Rust, variables are immutable by default, which means that once we give a value to a variable, we can't change it.
- If we want to change a variable's value, we need to declare it as mutable, using the `mut` keyword.
### Assigning a value to a variable
- To assign a value to a variable, we use the equals sign `=` and the value that we want to store in the variable.
- Here's an example:
```rust
let mut x: i32 = 5; // our earlier variable

x = 10; // we can change the value of x because we declared it as mutable
```
- This changes the value of `x` from `5` to `10`.
- If we try to change the value of `x` without declaring it as mutable, the program will not compile.
### Assigning to a variable after declaring it without a value
- This time, the `mut` keyword is not needed, because we are declaring the variable and giving it a value at the same time.
- Here's an example:
```rust
let y: i32;

y = 10; // we can change the value of y because we declared it without a value
```
## Constants
- Constants are similar to variables, but their value cannot change while the program is running.
- To declare a constant, we use the `const` keyword, followed by the name of the constant, and then a colon `:` and the data type of the constant.
- We can also add an equals sign `=` and the value that we want to store in the constant.
- Here's an example:
```rust
const MAX_POINTS: u32 = 100_000;
```
- Unlike normal variables, we must declare the data type of a constant.
- Also unlike normal variables, we can't use the `mut` keyword with constants, because they are always immutable.
- Constant don't have to be declared in a function, and they can be declared in any scope, including the global scope, unlike variables.
## Shadowing
- Shadowing is when we declare a new variable with the same name as an existing variable, and the new variable's value is used instead of the existing variable's value.
- We can shadow a variable by assigning a new value to it, and using the `let` keyword as usual.
- Here's an example:
```rust
let x: i32 = 5;

println!("The value of x is: {}", x);

let x = x + 1; // we can use the same name for the new variable

println!("The value of x after shadowing is: {}", x);

if x > 5 {
    let x = x * 2; // we can use the same name for the new variable again
    println!("The value of x inside the if block is: {}", x);
}

println!("The value of x after the if block is: {}", x);
```