/*
 * @lc app=leetcode id=1528 lang=rust
 *
 * [1528] Shuffle String
 */

// @lc code=start
impl Solution {
    pub fn restore_string(mut s: String, indices: Vec<i32>) -> String {
        let mut resVec:Vec<&str> = vec!["a" ;indices.len()];
        for n in 0..indices.len(){
            resVec[indices[n] as usize] = s.get(n..n+1).unwrap();
        }
        resVec.join("")
    }
}
// @lc code=end

