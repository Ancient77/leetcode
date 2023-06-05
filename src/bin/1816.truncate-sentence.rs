/*
 * @lc app=leetcode id=1816 lang=rust
 *
 * [1816] Truncate Sentence
 */

// @lc code=start
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let mut words = s.split(" ");
        let mut res = String::new();
        for i in 0..k{
            res.push_str(words.next().unwrap());
            res.push(' ');
        }
        res.trim_end().to_owned()
    }
}
// @lc code=end

