/*
 * @lc app=leetcode id=1518 lang=rust
 *
 * [1518] Water Bottles
 */

// @lc code=start
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut sum = num_bottles;
        while num_bottles>=num_exchange {
            sum+= num_bottles / num_exchange;
            num_bottles = num_bottles / num_exchange + num_bottles % num_exchange
        }
        sum
    }
}
// @lc code=end

