# Rustfmt
Rustfmt is a tool that automatically formats Rust code according to a set of style guidelines. It is a tool that is used to ensure that all Rust code is formatted in a consistent manner. This is important because it makes the code easier to read and understand, and it also helps to prevent bugs and other issues that can arise from poorly formatted code.
## Installing Rustfmt
To install Rustfmt, you can use the `rustup` command. Here's an example:
```sh
rustup component add rustfmt
```
This will install Rustfmt on your system, and you can use it to format your Rust code.

Don't worry though - rustfmt will probably be installed by default when you install Rust.
## Using Rustfmt
To use Rustfmt, you can use the `rustfmt` command, followed by the name of the file that you want to format. Here's an example:
```sh
rustfmt main.rs
```
To format all the Rust files in a directory, you can use the `--recursive` flag, like this:
```sh
rustfmt --recursive src/
```
or
```sh
rustfmt src/*
```
This will format all the Rust files in the `src` directory.
