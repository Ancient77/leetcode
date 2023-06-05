/*
 * @lc app=leetcode id=1295 lang=rust
 *
 * [1295] Find Numbers with Even Number of Digits
 */

// @lc code=start
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|x| x.to_string().len() % 2 == 0).count() as i32
    }
}
// @lc code=end

