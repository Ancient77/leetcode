/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut offset = 0;
        let len = nums.len();
        for n in 0..len{
            if(nums[n] == 0){
                offset+=1;
            }else {
                nums[n-offset] = nums[n];
            }
        }
        for x in 1..=offset{
            nums[len-x] = 0;
        }
    }
}
// @lc code=end

