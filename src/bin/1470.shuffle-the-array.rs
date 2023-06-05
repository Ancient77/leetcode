/*
 * @lc app=leetcode id=1470 lang=rust
 *
 * [1470] Shuffle the Array
 */

// @lc code=start
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut res:Vec<i32> = vec![];
        for x in 0..n as usize{
            res.push(nums[x]);
            res.push(nums[x+(n as usize)]);
        }
        res
    }
}
// @lc code=end

