/*
 * @lc app=leetcode id=1491 lang=rust
 *
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 */

// @lc code=start
impl Solution {
    pub fn average(mut salary: Vec<i32>) -> f64 {
        let mut sum:f64 = 0.0;
        salary.sort_unstable();
        for i in 1..salary.len()-1{
            sum +=salary[i] as f64;
        }
        sum / (salary.len()-2) as f64
    }
}
// @lc code=end

