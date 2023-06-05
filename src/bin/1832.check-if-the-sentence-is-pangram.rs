/*
 * @lc app=leetcode id=1832 lang=rust
 *
 * [1832] Check if the Sentence Is Pangram
 */

// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let x: HashSet<char> =sentence.chars().collect();
        x.len()== 26
    }
}
// @lc code=end

