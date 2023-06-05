/*
 * @lc app=leetcode id=2185 lang=rust
 *
 * [2185] Counting Words With a Given Prefix
 */

// @lc code=start
impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.iter().filter(|x| x.starts_with(&pref)).count() as i32
    }
}
// @lc code=end

