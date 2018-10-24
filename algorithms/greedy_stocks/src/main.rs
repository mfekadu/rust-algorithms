/* Rust things I learned
 *    - pointers (aka references) reminded me 
 *         Rust is strongly and statically typed
 *    - cargo gives warnings if names are not snake_case
 *    - return values can be "implicit" via just the last expression
 *    - "->" is used to define the return type
 */



/* the function "getMaxProfit" 
 * takes input "stockPrices" which is an immutable pointer to a i32
 *    https://stackoverflow.com/a/24833065/5411712
 *    I feel like this is the most basic thing, but still so hard...
 */
fn get_max_profit(stock_prices: &[i32]) -> i32 { 
  stock_prices[0]
}


fn main() {
  const NUM_STOCKS: usize = 6;
  /* this defines a fixed length immutable array of i32 values */ 
  let stock_prices: [i32; NUM_STOCKS] = [10, 7, 5, 8, 11, 9];
  println!("Hello, world!");
  println!("stock_prices: {:?}", stock_prices);
  println!("max_profit: {}", get_max_profit(&stock_prices));
}
