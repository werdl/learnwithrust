# Control Flow
Control flow is the order in which the computer executes statements in a program.
## If Statements
An `if` statement is a control structure that allows you to execute some code based on a condition. Here's an example:
```rust
fn main() { // here is the main function we mentioned earlier
    let number = 3; // we create a variable called number, and give it the value 3

    if number < 5 { // if the number is less than 5, `<` is the less-than operator, like in maths
        println!("The number is less than 5"); // if the condition is true, we print this message
    } else { // this is the `else` block, which is executed if the condition is false
        println!("The number is greater than or equal to 5"); // if the condition is false, we print this message
    }
}
```
When you run this program, you should see the following output:
```sh
The number is less than 5
```
### Else If
You can also have multiple conditions using `else if`:
```rust
fn main() {
    let number = 6;

    if number % 4 == 0 { // `%` is the modulo operator, which gives the remainder of a division
        println!("The number is divisible by 4");
    } else if number % 3 == 0 { // if the number is not divisible by 4, we check if it is divisible by 3
        println!("The number is divisible by 3");
    } else if number % 2 == 0 { // if the number is not divisible by 3, we check if it is divisible by 2
        println!("The number is divisible by 2");
    } else { // if the number is not divisible by 4, 3, or 2, we print this message
        println!("The number is not divisible by 4, 3, or 2");
    }
}
```
When you run this program, you should see the following output:
```sh
The number is divisible by 3
```
- Each `if` condition is checked in order, and if it is true, the corresponding block of code is executed, and the rest of the conditions are not checked. If none of the conditions are true, the `else` block is executed.
- This is why it only prints "The number is divisible by 3", and not "The number is divisible by 2", even though 6 is divisible by both 2 and 3.
- The `else if` and `else` blocks are optional, and you can have as many `else if` blocks as you like.
## Loops
Loops are a way to run the same code multiple times. There are three kinds of loops in Rust: `loop`, `while`, and `for`.
### `loop`
The `loop` keyword creates an infinite loop. Here's an example:
```rust
fn main() {
    let mut counter = 0; // we declare a mutable variable called counter, and give it the value 0

    loop { // this is the start of the loop
        counter += 1; // this is the same as `counter = counter + 1`, it adds 1 to the value of counter

        println!("The value of counter is: {}", counter); // we print the value of counter

        if counter == 10 { // if the value of counter is 10, we stop the loop
            break; // this is the `break` keyword, it stops the loop
        }
    }
}
```
When you run this program, you should see the following output:
```sh
The value of counter is: 1
The value of counter is: 2
...
The value of counter is: 9
The value of counter is: 10
```
### `while`
The `while` keyword creates a loop that runs as long as a condition is true. Here's an example:
```rust
fn main() {
    let mut number = 3;

    while number != 0 { // while the value of number is not 0
        println!("{}!", number); // we print the value of number

        number -= 1; // this is the same as `number = number - 1`, it subtracts 1 from the value of number
    }

    println!("LIFTOFF!!!"); // when the value of number is 0, we print this message
}
```
When you run this program, you should see the following output:
```sh
3!
2!
1!
LIFTOFF!!!
```
### `for`
The `for` keyword creates a loop that runs for each item in a collection. Here's an example:
```rust
fn main() {
    let a = [10, 20, 30, 40, 50]; // we create an array called a, and give it some values

    for element in a.iter() { // for each element in the array a, don't worry about `.iter()` for now, just know that it's needed to make the loop work
        println!("The value is: {}", element); // we print the value of the element
    }
}
```
When you run this program, you should see the following output:
```sh
The value is: 10
The value is: 20
The value is: 30
The value is: 40
The value is: 50
```