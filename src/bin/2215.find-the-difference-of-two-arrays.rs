/*
 * @lc app=leetcode id=2215 lang=rust
 *
 * [2215] Find the Difference of Two Arrays
 */


// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut resSetOne: HashSet<i32> = nums1.into_iter().collect();
        let mut resSetTwo: HashSet<i32> = nums2.into_iter().collect();
        vec![resSetOne.difference(&resSetTwo).cloned().collect(), resSetTwo.difference(&resSetOne).cloned().collect()]
        
    }
}
// @lc code=end

