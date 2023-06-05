/*
 * @lc app=leetcode id=682 lang=rust
 *
 * [682] Baseball Game
 */

// @lc code=start
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut out = Vec::new();
        for x in operations{
            match x.as_str(){
                "+" => {out.push(out[out.len()-1] + out[out.len()-2]);},
                "D" => {out.push(out[out.len()-1]*2);},
                "C" => {out.pop();},
                _ => {out.push(x.parse().unwrap())}
            }
        }
        out.iter().sum()
    }
}
// @lc code=end

