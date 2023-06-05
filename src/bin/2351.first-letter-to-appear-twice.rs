/*
 * @lc app=leetcode id=2351 lang=rust
 *
 * [2351] First Letter to Appear Twice
 */


// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut set = HashSet::with_capacity(s.len());
        for c in s.chars(){
            if(!set.insert(c)){
                return c;
            }
        }
        panic!();
    }
}
// @lc code=end

