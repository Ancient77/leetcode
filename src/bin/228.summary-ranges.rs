/*
 * @lc app=leetcode id=228 lang=rust
 *
 * [228] Summary Ranges
 */

use std::fmt::format;

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        let mut n = 0;
        while n < nums.len() {
            let a = nums[n];
            
            let mut b = None;
            while nums.get(n+1).unwrap_or(&-1) - b.unwrap_or(a) == 1 {
                n = n+1;
                b = Some(nums[n])
            }
            match b {
                None => res.push(format!("{a}")),
                Some(x) => res.push(format!("{a}->{x}"))
            }
            n=n+1;
        }
        res
    }
}
// @lc code=end

