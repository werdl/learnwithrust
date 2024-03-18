# Data Types
Data types are a way to classify different types of data, and to tell the computer how to work with that data. In Rust, every value has a data type, and every data type has a set of operations that can be performed on it.
## Why bother?
- Data types make it easier to understand what a program is doing
- They also make it faster to run, because the computer doesn't need to check what kind of data it's working with
- They also make it safer to run, because the computer can check that the operations being performed on the data are valid
## What are the basic data types?
### Integers
- Whole numbers
- Can be of different sizes (in terms of how many bits they take up in memory)
#### Signed
- Can be positive or negative
- Types:
    - `i8` - 8-bit signed integer
    - `i16` - 16-bit signed integer
    - `i32` - 32-bit signed integer (this is the default)
    - `i64` - 64-bit signed integer
    - `i128` - 128-bit signed integer
    - `isize` - the size of the pointer for the current machine (32-bit on 32-bit systems, 64-bit on 64-bit systems)
#### Unsigned
- Can only be positive (or zero)
- Types:
    - `u8` - 8-bit unsigned integer
    - `u16` - 16-bit unsigned integer
    - `u32` - 32-bit unsigned integer
    - `u64` - 64-bit unsigned integer
    - `u128` - 128-bit unsigned integer
    - `usize` - the size of the pointer for the current machine (32-bit on 32-bit systems, 64-bit on 64-bit systems)
### Floating-point numbers
- Numbers with a decimal point
- Can be of different sizes (in terms of how many bits they take up in memory)
- They are always signed, so they can be positive or negative
- Types:
    - `f32` - 32-bit floating-point number
    - `f64` - 64-bit floating-point number (this is the default)
### Booleans
- Can be either `true` or `false`
- Type:
    - `bool`
### Characters
- A single Unicode (a standard for representing text in most of the world's writing systems) character
- Type:
    - `char`
### Strings
- A sequence of Unicode characters
- Type:
    - `str` - a string slice (a reference (a location of the slice in your RAM) to a sequence of Unicode characters)
    - `String` - a string (a sequence of Unicode characters stored in memory that the program can use, and change more easily than a string slice)

Now that we know what the basic data types are, let's learn how to use them in our programs with [variables](variables.md)!