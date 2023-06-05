/*
 * @lc app=leetcode id=2108 lang=rust
 *
 * [2108] Find First Palindromic String in the Array
 */

// @lc code=start
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words.iter().find(|x| {
            for i in 0..x.len()/2{
                if(x.get(i..i+1).unwrap() != x.get(x.len()-1-i..x.len()-i).unwrap()){
                    return false;
                }
            }
            true
        }).unwrap_or(&"".to_owned()).to_string()
    }
}
// @lc code=end

