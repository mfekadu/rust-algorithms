/* Rust things I learned
 *    - pointers (aka references) reminded me 
 *         Rust is strongly and statically typed
 *    - cargo gives warnings if names are not snake_case
 *    - return values can be "implicit" via just the last expression
 *    - "->" is used to define the return type
 */



/* the function "getMaxProfit" 
 * takes input "stockPrices" which is a mutable pointer to a i32
 *    https://stackoverflow.com/a/24833065/5411712
 *    I feel like this is the most basic thing, but still so hard...
 */
fn get_max_profit(stock_prices: &mut [i32]) -> i32 { 
  stock_prices[0] /* dereference the pointer at 0??? */
}

fn main() {
  let mut stock_prices = [10, 7, 5, 8, 11, 9];
  println!("Hello, world!");
  println!("{}", get_max_profit(&mut stock_prices));
}
