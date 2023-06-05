/*
 * @lc app=leetcode id=1920 lang=rust
 *
 * [1920] Build Array from Permutation
 */

// @lc code=start
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res:Vec<i32> = vec![];
        for x in &nums{
            res.push(nums[*x as usize] as i32);
        }
        res
    }
}
// @lc code=end

