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
## Else If
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
