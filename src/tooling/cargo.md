# Cargo
Cargo is a piece of Rust software to more easily manage large projects. It can build your code, and also incorporate other people's code - dependencies - into your project. It also has a number of other features that make it easier to manage your project.
## Creating a New Project
To create a new project with Cargo, you can use the `cargo new` command. This will create a new directory with the name of your project, and a `Cargo.toml` file inside it. Here's an example:
```sh
cargo new my_project
```
This will create a new directory called `my_project`, and a `Cargo.toml` file inside it. The `Cargo.toml` file is where you can configure your project, and add dependencies to it.

To check if this has worked, the `tree` command can be used to show the directory structure. If you don't have `tree` installed, you can install it with `sudo apt install tree` on Ubuntu, or `brew install tree` on macOS.

Here's an example of the directory structure:
```sh
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```
Before running the `tree` command, you need to `cd` into the `my_project` directory.
## Building a Project
To build a project with Cargo, you can use the `cargo build` command. This will compile your code and create an executable file in the `target/debug` directory. Here's an example:
```sh
cargo build
```
This will compile the code in the `src/main.rs` file, and create an executable file called `my_project` in the `target/debug` directory.

To run the executable file, you can use the `./` command, followed by the name of the executable file. Here's an example:
```sh
./target/debug/my_project
```
## Running a Project
To run a project with Cargo, you can use the `cargo run` command. This will compile your code and run the executable file in the `target/debug` directory. Here's an example:
```sh
cargo run
```
This will compile the code in the `src/main.rs` file, and run the executable file called `my_project` in the `target/debug` directory.

You can also pass arguments to the executable file, like this:
```sh
cargo run -- arg1 arg2 arg3
```
This will pass the arguments `arg1`, `arg2`, and `arg3` to the executable file.
Don't worry if you don't understand everything in this chapter yet. We will cover all of these concepts in more detail in later chapters.