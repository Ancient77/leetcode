/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        
        let mut max:i32 = (nums.len() * (nums.len()+1)) as i32 / 2;
        for x in nums{
            max -= x;
        }
        max
    }
}
// @lc code=end

