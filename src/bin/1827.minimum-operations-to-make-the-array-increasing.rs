/*
 * @lc app=leetcode id=1827 lang=rust
 *
 * [1827] Minimum Operations to Make the Array Increasing
 */

// @lc code=start
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 1..nums.len(){
            if(nums[i] <= nums[i-1]){
                res+= nums[i-1] + 1 - nums[i];
                nums[i] = nums[i-1] + 1;
            }
        }
        res
    }
}
// @lc code=end

