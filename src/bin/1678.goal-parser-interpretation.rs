/*
 * @lc app=leetcode id=1678 lang=rust
 *
 * [1678] Goal Parser Interpretation
 */

// @lc code=start
impl Solution {
    pub fn interpret(mut command: String) -> String {
        command.replace("()", "o").replace("(", "").replace(")", "")
        
    }
}
// @lc code=end

