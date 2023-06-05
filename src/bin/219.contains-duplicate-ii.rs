/*
 * @lc app=leetcode id=219 lang=rust
 *
 * [219] Contains Duplicate II
 */



// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hash = HashMap::new();
        for x in 0..nums.len() {
              match hash.insert(nums[x],x){
                Some(n) => if(((n-x) as i32).abs() <= k || ((x-n) as i32).abs() <= k){return true;}
                None => (),
              }
        }
        false
    }
}
// @lc code=end

