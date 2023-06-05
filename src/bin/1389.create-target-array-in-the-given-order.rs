/*
 * @lc app=leetcode id=1389 lang=rust
 *
 * [1389] Create Target Array in the Given Order
 */

// @lc code=start
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut resVec = Vec::with_capacity(nums.len());
        for n in 0..nums.len(){
            resVec.insert(index[n] as usize, nums[n]);
        }
        resVec
    }
}
// @lc code=end

