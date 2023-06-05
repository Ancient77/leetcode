/*
 * @lc app=leetcode id=540 lang=rust
 *
 * [540] Single Element in a Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        //O(n) but should be O(log(n))
        let mut out = 0;
        for n in nums{
            out = out ^ n;
        }
        out
    }
}
// @lc code=end

