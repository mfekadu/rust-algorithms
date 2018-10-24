# rust-algorithms

learning algorithms and rust-lang at the same time

# First start with the [rust-lang book][1]

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
* creates a new directory called "hello_cargo" (or any name you want) that serves as a nicely packaged project folder where you can [build and run using cargo][2]
  * **cool stuff!**

## [more exploration of Rust here][3]
  * see [this file][4] especially 


# Algorithms
## Greedy
## Graphs
## Etc



[1]: https://doc.rust-lang.org/book/
[2]: https://doc.rust-lang.org/book/2018-edition/ch01-03-hello-cargo.html#building-and-running-a-cargo-project
[3]: https://github.com/mfekadu/rust-algorithms/tree/master/learning_rust
[4]: https://github.com/mfekadu/rust-algorithms/blob/master/learning_rust/guessing_game/src/main.rs
