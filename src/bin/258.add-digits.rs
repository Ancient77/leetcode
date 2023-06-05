/*
 * @lc app=leetcode id=258 lang=rust
 *
 * [258] Add Digits
 */

// @lc code=start
impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
       let x = num % 9;
       if( x == 0 &&num != 0 
    ){
        return 9;
       }
       return x;
    }
}
// @lc code=end

