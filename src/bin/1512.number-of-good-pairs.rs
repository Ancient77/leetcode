/*
 * @lc app=leetcode id=1512 lang=rust
 *
 * [1512] Number of Good Pairs
 */

// @lc code=start
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for x in 0..nums.len() {
            for y in x+1..nums.len() {
                if(nums[x] == nums[y]){
                    sum+=1;
                }
            }
        }
        sum
    }
}
// @lc code=end

