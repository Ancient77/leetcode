/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let len = s.len()-1;
        for i in 0..(len+1)/2{
            let buf = s[len-i];
            s[len-i] = s[i];
            s[i] = buf;
        }
    }
}
// @lc code=end

