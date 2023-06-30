/*
 * @lc app=leetcode id=1342 lang=rust
 *
 * [1342] Number of Steps to Reduce a Number to Zero
 */

// @lc code=start
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        31 - num.leading_zeros() as i32 + num.count_ones().max(1) as i32
    }
}
// @lc code=end

