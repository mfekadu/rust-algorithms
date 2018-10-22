# rust-algorithms

learning algorithms and rust-lang at the same time

# First start with the [rust-lang book](https://doc.rust-lang.org/book/)

## basics
### Make file, write hello world

`> vim main.rs`
```rust
fn main() {
  // fun fact the "!" character allows tells Rust to call the println "macro"
  // yup, println is a Rust "macro" not a function
  println!("Hello World");
}
```
> **Note**: the file does not need be called "main.rs" but **there must be a _main function_**

> you can run `rustc --explain E0601` in the command line to get an explanation... **that's awesome!!**

### Compile and run
```bash
>rustc main.rs
>ls
main    main.rs
>./main
Hello World
```

### Cargo is cool
`cargo --version` gives you the version

```bash
>cargo new hello_cargo
     Created binary (application) `hello_cargo` project
```
* creates a new directory called "hello_cargo" (or any name you want) that serves as a nicely packaged project folder where you can [build and run using cargo](https://doc.rust-lang.org/book/2018-edition/ch01-03-hello-cargo.html#building-and-running-a-cargo-project)
  * **cool stuff!**

## [more exploration of Rust here](/learning_rust)
