/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */



// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        let mut hash = HashSet::new();
        for x in nums {
              if !hash.insert(x){
                return true;
              }
        }
        false
    }
}
// @lc code=end

