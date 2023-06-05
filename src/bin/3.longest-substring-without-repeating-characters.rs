/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */



// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut best = 0;
        let mut hash = HashMap::new();
        for (n,x) in s.chars().enumerate(){
            match hash.insert(x, n) {
                None => (),
                Some(x) => {
                    best = if (hash.len() > best) {hash.len()} else {best};
                    hash.retain(|_, v| *v > x);
                }
            }
        }
        if (best > hash.len()){best as i32} else {hash.len() as i32}
        
    }
}
// @lc code=end

