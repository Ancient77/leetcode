/*
 * @lc app=leetcode id=561 lang=rust
 *
 * [561] Array Partition
 */

// @lc code=start
impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.sort_unstable();
        for i in 0..nums.len()/2{
            res+= nums[i*2].min(nums[i*2+1]);
        }
        res
    }
}
// @lc code=end

