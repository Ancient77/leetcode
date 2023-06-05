/*
 * @lc app=leetcode id=1672 lang=rust
 *
 * [1672] Richest Customer Wealth
 */

// @lc code=start
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut maxW = 0;
        for x in accounts{
            maxW = maxW.max(x.iter().sum());
        }
        maxW
    }
}
// @lc code=end

