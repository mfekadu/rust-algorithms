# rust-algorithms

learning algorithms and rust-lang at the same time

## First start with the [rust-lang book](https://doc.rust-lang.org/book/)

### basics
**Make file, write hello world**

`> vim main.rs`
```rust
fn main() {
  // fun fact the "!" character allows tells Rust to call the println "macro"
  // yup, println is a Rust "macro" not a function
  println!("Hello World");
}
```
**Compile and run**
```
>rustc main.rs
>ls
main    main.rs
>./main
Hello World
```
