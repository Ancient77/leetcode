/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */

// @lc code=start
impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        num ^ (i32::MAX >> (31.min(num.leading_zeros()-1)))
    }
}
// @lc code=end

