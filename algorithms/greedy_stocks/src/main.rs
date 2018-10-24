/* Rust things I learned
 *    - pointers (aka references) reminded me 
 *         Rust is strongly and statically typed
 *    - cargo gives warnings if names are not snake_case
 *    - return values can be "implicit" via just the last expression
 *    - "->" is used to define the return type
 */

/* https://www.interviewcake.com/question/c/stock-price?section=greedy&course=fc1 */

/* the function "get_max_profit" 
 * takes input "stock_prices" which is an immutable pointer to a i32
 *    https://stackoverflow.com/a/24833065/5411712
 *    I feel like this is the most basic thing, but still so hard...
 * takes input "num_stocks" which is a usize
 *    for the length of the stock_prices array. 
 * outputs an i32 for the maximum profit after buying low and selling high.
 */
fn get_max_profit(stock_prices: &[i32], num_stocks: usize) -> i32 { 
  let mut min_price:i32;
  let mut max_profit:i32;

  /* make sure we have at least 2 stocks, using the assert! macro */
  assert!(num_stocks >= 2);

  /* goal is to greedily update min_price and max_profit
   * so initialize with first possible values
   */
  min_price = stock_prices[0];
  max_profit = stock_prices[1] - stock_prices[0];

  /* start at the second index
   * selling at first index is illegal, because buying must happen first
   *    and beacuase buying and selling at the same time is illegal
   * problem statement asks for negative i32 value if array is decreasing.
   */
  /* Rust control flow:
   * https://doc.rust-lang.org/book/2018-edition/ch03-05-control-flow.html
   */ 
  for t in 1..num_stocks { /* from t=1 to t=num_stocks */
    let current_price:i32 = stock_prices[t];

    let potential_profit = current_price - min_price;

    /* found a lengthy debate on the lack of ternary operator in Rust 
     *    https://github.com/rust-lang/rfcs/issues/1362
     */

    /* update max to try to do better */
    /* I purposely did not have `use std::cmp;` for funsies */
    max_profit = std::cmp::max(max_profit, potential_profit);

    /* update min, to always have the smallest so far */
    min_price = std::cmp::min(min_price, current_price);
  } 

  max_profit /* optionally say return max_profit; */
}


fn main() {
  const NUM_STOCKS: usize = 6;

  /* this defines a fixed length immutable array of i32 values */ 
  let stock_prices: [i32; NUM_STOCKS] = [10, 7, 5, 8, 11, 9];

  println!("stock_prices = {:?}", stock_prices);

  println!("max_profit = {}", get_max_profit(&stock_prices, NUM_STOCKS));
  // expect max_profit = 6 (buying for $5 and selling for $11)
}
