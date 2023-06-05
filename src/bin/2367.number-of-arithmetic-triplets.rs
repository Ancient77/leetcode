/*
 * @lc app=leetcode id=2367 lang=rust
 *
 * [2367] Number of Arithmetic Triplets
 */

// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut set: HashSet<i32> = nums.iter().cloned().collect();
        let mut c = 0;

        for n in nums {
            if (set.contains(&(n - diff)) && set.contains(&(n + diff))) {c+=1}
        }
        c
    }
}
// @lc code=end
