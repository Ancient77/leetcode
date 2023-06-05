/*
 * @lc app=leetcode id=1480 lang=rust
 *
 * [1480] Running Sum of 1d Array
 */

// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut buf = 0;
        let mut resVec = Vec::with_capacity(nums.len());
        for x in nums{
            buf+=x;
            resVec.push(buf);
        }
        resVec
    }
}
// @lc code=end

