/*
 * @lc app=leetcode id=674 lang=rust
 *
 * [674] Longest Continuous Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut out = 0;
        let mut cur = 1;
        for n in 1..nums.len(){
            if(nums[n] > nums[n-1]){cur+=1;}
            else{
                out =if (cur > out)  {cur} else {out};
                cur = 1;
            }
        }
        if(cur > out){
            return cur;
        }
        out
    }
}
// @lc code=end

