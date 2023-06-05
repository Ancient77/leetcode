/*
 * @lc app=leetcode id=1961 lang=rust
 *
 * [1961] Check If String Is a Prefix of Array
 */

// @lc code=start
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        if(s == words[0]){return true;}
        let mut cocWord = String::new();
        for word in words{
            cocWord.push_str(&word);
            if(s == cocWord){
                return true;
            }
        }
        false
    }
}
// @lc code=end

