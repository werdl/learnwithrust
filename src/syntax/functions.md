# Functions
Functions are used to group sections of code together for later usage.
## Declaring a Function
To declare a function, we use the `fn` keyword, followed by the name of the function, and then a set of parentheses `( )`. Inside the parentheses, we can add a list of parameters, which are the inputs to the function. After the parentheses, we add a set of curly braces `{ }`, and inside the curly braces, we add the code that the function will run. Here's an example:
```rust
fn add(x: i32, y: i32) { // we declare a function called add, and give it two parameters, x and y
    println!("The sum of {} and {} is: {}", x, y, x + y); // we print the sum of x and y
}

fn main() {
    add(5, 10); // we call the add function, and give it the values 5 and 10
}
```
When you run this program, you should see the following output:
```sh
The sum of 5 and 10 is: 15
```
## Returning a Value from a Function
To return a value from a function, we use the `->` symbol, followed by the data type of the value that we want to return. We also use the `return` keyword, followed by the value that we want to return. Here's an example:
```rust
fn add(x: i32, y: i32) -> i32 { // we declare a function called add, and give it two parameters, x and y, and tell it to return a 32-bit signed integer
    x + y // we return the sum of x and y
}

fn main() {
    let sum = add(5, 10); // we call the add function, and give it the values 5 and 10, and store the result in a variable called sum
    println!("The sum of 5 and 10 is: {}", sum); // we print the value of sum
}
```
When you run this program, you should see the following output:
```sh
The sum of 5 and 10 is: 15
```
As you can see, the last line of `add` has no semicolon `;`, which means that it is an expression that returns a value. This is why we don't need to use the `return` keyword in this case.
> We can also use the `return` keyword if we want to return a value from a function early, before the end of the function's code, or if we want to make the code more readable for non-Rust programmers.
## Expressions vs Statements
In Rust, expressions are pieces of code that return a value, and statements are pieces of code that don't return a value. Here are some examples of expressions and statements:
### Expressions
- `5 + 10` - returns the value `15`
- `add(5, 10)` - returns the value `15`
- `x + y` - returns the value of `x` plus the value of `y`
### Statements
- `let x = 5;` - doesn't return a value
- `let y = add(5, 10);` - doesn't return a value
- `x + y;` - doesn't return a value
## Function Bodies
The body of a function is the code inside the curly braces `{ }`. The last line of the body is an expression, and the value of that expression is the value that the function returns. Here's an example:
```rust
fn countdown_from(number: i32) -> i32 {
    let mut counter = number; // we declare a mutable variable called counter, and give it the value of number

    while counter != 0 { // while the value of counter is not 0
        println!("{}!", counter); // we print the value of counter

        counter -= 1; // this is the same as `counter = counter - 1`, it subtracts 1 from the value of counter
    }

    println!("LIFTOFF!!!"); // when the value of counter is 0, we print this message

    counter // the value of counter is the value that the function returns
}

fn main() {
    let result = countdown_from(3); // we call the countdown_from function, and give it the value 3, and store the result in a variable called result
    println!("The value of result is: {}", result); // we print the value of result
}
```
When you run this program, you should see the following output:
```sh
3!
2!
1!
LIFTOFF!!!
```
## Function Parameters
Function parameters are the inputs to the function. We can give them names, and we must also give them data types. Here's an example:
```rust
fn greet(name: &str) { // we declare a function called greet, and give it a parameter called name, which is a reference to a string slice
    println!("Hello, {}!", name);
}

fn irrelevant_parameter(_: i32) { // we declare a function called irrelevant_parameter, and give it a parameter called _, which means that we don't care about the value of the parameter
    println!("I don't care about the value of the parameter!");
}
```