# Unions
Unions are a way to store different types in the same memory location. They are similar to structs, but with a few key differences. In a struct, each field has its own memory location, but in a union, all fields share the same memory location. This means that only one field can be used at a time.

## What is a Union?
A union is a data structure that can store different types in the same memory location. This is useful when you want to store different types of data in the same variable, but only use one type at a time.

Here is a handy analogy to help you understand unions:
Imagine you have a box that can hold either a book or a toy. You can only have one item in the box at a time, but you can switch between the book and the toy whenever you want. When you put a book in the box, the toy is removed, and vice versa. When you check the box, you will only find either the book or the toy, not both.

## When to Use Unions
Unions are useful when you want to store different types of data in the same memory location, but only use one type at a time. For example, you might use a union to store different types of numbers (integers, floats, etc.) in the same variable, but only use one type at a time.

Example:
```rust
union Number {
    integer: i32,
    float: f32,
}


fn main() {
    let num = Number { integer: 42 };
    unsafe {
        println!("The number is: {}", num.integer);

        println!("The number is: {}", num.float); // This will print garbage data
    }
}
``` 

### But why? Why would you want to do this?
Unions are useful when you want to save memory by storing different types in the same memory location. For example, if you have a variable that can be either an integer or a float, you can use a union to store both types in the same memory location. This can save memory because you only need to allocate memory for the largest type in the union.

## Safety
Unions are unsafe in Rust because they can lead to undefined behavior if used incorrectly. For example, if you try to access a field in a union that is not currently in use, you will get garbage data. This is because unions do not keep track of which field is currently in use, so it is up to you to keep track of this information.