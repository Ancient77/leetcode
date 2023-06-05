/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut vec = vec![0,1,2];
        while n > (vec.len() -1 ) as i32 {
            vec.push(vec[vec.len()-1] + vec[vec.len() -2]);
        }
        vec[n as usize]
    }
}
// @lc code=end

