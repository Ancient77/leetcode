/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let aInt = i128::from_str_radix(&a, 2).unwrap();
        let bInt = i128::from_str_radix(&b, 2).unwrap();
        let sum = aInt + bInt;
        format!("{sum:b}")
    }
}
// @lc code=end

