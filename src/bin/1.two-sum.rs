/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for x in 0..nums.len() {
            for n in 0..nums.len() {
                if(x == n) {continue;}
                if(nums[x] + nums[n] == target){
                    return vec![x as i32 ,n as i32];
                }
            }
        }
        vec![]
    }
}
// @lc code=end

