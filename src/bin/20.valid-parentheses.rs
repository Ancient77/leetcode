/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for k in s.chars(){
            match k {
                '(' => stack.push('('),
                '[' => stack.push('['),
                '{' => stack.push('{'),
                ')' => if stack.pop() != Some('(') {return false},
                ']' => if stack.pop() != Some('[') {return false},
                '}' => if stack.pop() != Some('{') {return false},
                _ => ()
            }
            
        }
        stack.len() == 0
    }
}
// @lc code=end

