/*
 * @lc app=leetcode id=2469 lang=rust
 *
 * [2469] Convert the Temperature
 */

// @lc code=start
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.0]
    }
}
// @lc code=end

