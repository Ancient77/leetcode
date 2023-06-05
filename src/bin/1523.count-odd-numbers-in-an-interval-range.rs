/*
 * @lc app=leetcode id=1523 lang=rust
 *
 * [1523] Count Odd Numbers in an Interval Range
 */

// @lc code=start
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (((((high-low) / 2) as f64).floor() as i32) + ((low % 2).max(high % 2))) as i32
    }
}
// @lc code=end

