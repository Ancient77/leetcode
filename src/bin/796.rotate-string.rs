/*
 * @lc app=leetcode id=796 lang=rust
 *
 * [796] Rotate String
 */

// @lc code=start
impl Solution {
    pub fn rotate_string(s: String, mut goal: String) -> bool {
        if(s.len() > goal.len()){return false;}
        goal = format!("{}{}",goal,goal);
        return goal.contains(&s);
        
    }
}
// @lc code=end

