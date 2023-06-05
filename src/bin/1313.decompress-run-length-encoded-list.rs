/*
 * @lc app=leetcode id=1313 lang=rust
 *
 * [1313] Decompress Run-Length Encoded List
 */

// @lc code=start
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut resVec = Vec::new();
        for n in 0..nums.len() / 2{
            for i in 0..nums[2*n]{
                resVec.push(nums[n*2+1]);
            }
        }
        resVec
    }
}
// @lc code=end

