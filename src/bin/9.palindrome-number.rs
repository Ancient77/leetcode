/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if(x<0) {return false}
        if(x.to_string() == x.to_string().chars().rev().collect::<String>()){return true}
        false
    }
}
// @lc code=end

