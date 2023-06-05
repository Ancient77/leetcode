/*
 * @lc app=leetcode id=520 lang=rust
 *
 * [520] Detect Capital
 */

// @lc code=start
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        if(word == word.to_uppercase()){return true;}
        if(word == word.to_lowercase()){return true;}
        let (one,other) = word.split_at(1);
        if(one.to_uppercase() == one && other.to_lowercase() == other){return true;}
        false
    }
}
// @lc code=end

