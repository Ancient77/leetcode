/*
 * @lc app=leetcode id=2529 lang=rust
 *
 * [2529] Maximum Count of Positive Integer and Negative Integer
 */

use itertools::Itertools;

// @lc code=start
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut nums2 = nums.iter().filter(|z| **z!=0).collect::<Vec<&i32>>();
        let x:usize  = nums2.binary_search(&&0).unwrap_or_else(|y| return y);
        x.max(nums2.len() - x) as i32
    }
}
// @lc code=end

