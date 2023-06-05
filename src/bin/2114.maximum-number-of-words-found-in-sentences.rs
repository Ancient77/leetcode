/*
 * @lc app=leetcode id=2114 lang=rust
 *
 * [2114] Maximum Number of Words Found in Sentences
 */

// @lc code=start
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max = 0;
        for x in sentences{
            max = x.split(" ").collect::<Vec<&str>>().len().max(max);
        }
        max as i32
    }
}
// @lc code=end

