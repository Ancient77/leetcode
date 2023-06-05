/*
 * @lc app=leetcode id=461 lang=rust
 *
 * [461] Hamming Distance
 */

// @lc code=start
impl Solution {
    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        let mut c = 0;
        while x != 0 || y != 0{
            if(x >> 1 != y >> 1){
                c+=1;
            }
            x = x >> 1;
            y = y >> 1;
        }
        c
    }
}
// @lc code=end

