/*
 * @lc app=leetcode id=657 lang=rust
 *
 * [657] Robot Return to Origin
 */

// @lc code=start
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut pos = vec![0,0];
        for n in moves.chars() {
            match n{
                'U' => {pos[0]+=1},
                'D' => {pos[0]-=1},
                'L' => {pos[1]+=1},
                'R' => {pos[1]-=1},
                _ => {}
            }
        }
        pos[0] == 0 && pos[1] == 0
    }
}
// @lc code=end

