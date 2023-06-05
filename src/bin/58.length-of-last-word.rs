/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
       let mut i = s.trim().len() - 1;
        while (s.trim().as_bytes()[i] != b' '){
            if(i == 0){return s.trim().len() as i32;}
            i-= 1;
       }
       (s.trim().len()-1 - i )as i32
    }
}
// @lc code=end

