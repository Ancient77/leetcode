/*
 * @lc app=leetcode id=2395 lang=rust
 *
 * [2395] Find Subarrays With Equal Sum
 */


// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(100);
        for i in 1..nums.len(){
            let ele = nums[i] + nums[i-1];
            if set.contains(&ele){return true;}
            set.insert(ele);
        }
        false
    }
}
// @lc code=end

