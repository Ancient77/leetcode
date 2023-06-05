/*
 * @lc app=leetcode id=1684 lang=rust
 *
 * [1684] Count the Number of Consistent Strings
 */

// @lc code=start
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        words.iter().filter(|x| x.chars().all(|y| allowed.contains(y))).count() as i32
    }
}
// @lc code=end

