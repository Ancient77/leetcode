/*
 * @lc app=leetcode id=1431 lang=rust
 *
 * [1431] Kids With the Greatest Number of Candies
 */

// @lc code=start
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        let mut resVec = Vec::with_capacity(candies.len());
        for x in candies{
            resVec.push(x+extra_candies >= max);
        }
        resVec
    }
}
// @lc code=end

