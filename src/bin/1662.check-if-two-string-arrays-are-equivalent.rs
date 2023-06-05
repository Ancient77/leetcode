/*
 * @lc app=leetcode id=1662 lang=rust
 *
 * [1662] Check If Two String Arrays are Equivalent
 */

// @lc code=start
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let mut wordOne = word1.join("").chars().collect::<Vec<char>>();
        let mut wordTwo = word2.join("").chars().collect::<Vec<char>>();
        if(wordOne.len() != wordTwo.len()){
            return false;
        }
        for i in 0..wordOne.len() {
            if(*wordOne.get(i).unwrap() != *wordTwo.get(i).unwrap()){
                return false;
            }
        }
        true
    }
}
// @lc code=end

