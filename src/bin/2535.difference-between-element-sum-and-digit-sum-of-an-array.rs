/*
 * @lc app=leetcode id=2535 lang=rust
 *
 * [2535] Difference Between Element Sum and Digit Sum of an Array
 */

// @lc code=start
impl Solution {
    pub fn difference_of_sum(mut nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut digitSum = 0;
        for mut x in nums{
            sum+=x;
            while x > 0 {
                digitSum+=x%10;
                x/=10;
            }
        }
        (sum-digitSum).abs()
    }
}
// @lc code=end

