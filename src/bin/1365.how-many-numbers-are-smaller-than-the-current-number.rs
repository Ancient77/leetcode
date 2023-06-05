/*
 * @lc app=leetcode id=1365 lang=rust
 *
 * [1365] How Many Numbers Are Smaller Than the Current Number
 */

// @lc code=start
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut resVec = Vec::with_capacity(nums.len());
        for n in 0..nums.len(){
            resVec.push(0);
            for i in 0..nums.len(){
                if(nums[n] > nums[i]){
                    resVec[n]+=1;
                }
            }
        }
        resVec
    }
}
// @lc code=end

