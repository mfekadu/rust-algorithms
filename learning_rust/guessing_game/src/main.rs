extern crate rand; /* not built-in to rust, brought in by Cargo */

use std::io; /* standard input/output library, like #include <stdio.h> */
use rand::Rng;
/* Ordering is an enum with variants Less, Greater and Equal */
use std::cmp::Ordering;


fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1,101);

  /* it is not a secret if you tell anyone ;) */
  // println!("The secret number is: {}", secret_number);

  /* like while(true) loop */
  loop {

    println!("Please input your guess.");

    /* define a new, mutable, empty string */
    /* the "::" means "new" is "associated" with "String"
     * because "String" is a "type" 
     * rather than a "particular instance of a type"
     */
    let mut guess = String::new();

    /* again "stdin" is associated function of "io" */
    /* if not `use std::io` then need say `std::io::stdin` */
    /* stdin() returns "instance" of `std:io::Stdin` type */
    /* that instance has the read_line() function
     *    which takes in a pointer to a mutable String buffer 
     *    and it will fill the bufer with the user input
     *    but `&guess` would be an IMMUTABLE pointer
     *    so `&mut guess` gives you a MUTABLE pointer
     */
    io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
    /* lastly the error handling:
     *    read_line() retuns a type "std::io::Result"
     *    "io::Result" types are enums with Ok or Err
     *    expect("str") 
     *     will safely unwrap an Ok and panic with "str" if Err
     */

    /* print with string format */
    println!("You guessed: {}", guess);

    /* convert guess to a 32 bit unsigned int */
    /* NOTE: Rust is strongly and statically typed */
    /* trim() will eliminate the '\n'
     * parse() will convert to some kind of number
     * the ':' after guess tells Rust to annotate the variable type
     * expect() will handle if say "A*$" was the guess
     *    update....
     *    instead of that, handle each variant of io::Result
     *    because expect() will just crash with given message and backtrace
     */
    let guess: u32 = match guess.trim().parse() {
      /* NOTE: this is a match-expression for error-handling */
      Ok(num) => num,     /* put the Ok(result) into `guess` variable */
      Err(_) => { 
        println!("Please input a number");
        continue /* next iteration of loop */
      }
    };

    /* soo..I suppose you can do .cmp() on any "std" type? or subtype?? */
    /* "match" is a control-flow operator, kinda like switch-case, but not
     *   "a match expression is made of 'arms'"
     *   "an arm consists of a 'pattern'"
     *   "Rust takes the value given to match and looks through each pattern"
     */
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),  /* guess <  secret */
      Ordering::Greater => println!("Too big!"), /* guess >  secret */
      Ordering::Equal => {                       /* guess == secret */
        println!("You got it!");
        break; /* break out of loop */
      } 
    }
  }
}
