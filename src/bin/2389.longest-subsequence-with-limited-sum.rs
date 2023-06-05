/*
 * @lc app=leetcode id=2389 lang=rust
 *
 * [2389] Longest Subsequence With Limited Sum
 */

// @lc code=start
impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let mut res = vec![];
        for q in queries{
            let mut count = 0;
            let mut sum = q;
            for x in &nums{
                sum = sum - x;
                if(sum < 0){
                    break;
                }
                count+=1;
            }
            res.push(count);
        }        
        res
    }
}
// @lc code=end

