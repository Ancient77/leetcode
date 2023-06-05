/*
 * @lc app=leetcode id=2348 lang=rust
 *
 * [2348] Number of Zero-Filled Subarrays
 */

// @lc code=start
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut sum:i64 = 0;
        let mut zeroCount:i64 = 0;
        for n in nums{
            if(n ==0){
                zeroCount+=1;
            }else{
                if(zeroCount > 0){
                    sum += (1..=zeroCount).sum::<i64>();
                    zeroCount=0;
                }
            }
        }
        sum+=(1..=zeroCount).sum::<i64>();
        sum
    }
}

// @lc code=end

