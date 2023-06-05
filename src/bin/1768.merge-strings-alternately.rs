/*
 * @lc app=leetcode id=1768 lang=rust
 *
 * [1768] Merge Strings Alternately
 */

// @lc code=start
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();
        let mut resVec = Vec::with_capacity(word1.len() + word2.len());
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(x), Some(y)) => {resVec.push(x);resVec.push(y)},
                (Some(x), None) | (None, Some(x)) => resVec.push(x),
                (None,None) => break
            }
        }
        resVec.iter().collect()
    }
}
// @lc code=end

