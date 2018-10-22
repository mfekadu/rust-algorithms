extern crate rand; /* not built-in to rust, brought in by Cargo */

use std::io; /* standard input/output library, like #include <stdio.h> */
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1,101);

  println!("The secret number is: {}", secret_number);

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

//  if secret_number == guess {
 //   println!("you got it!");
//  } else {
 //   println!("wrong!");
//  }
}
