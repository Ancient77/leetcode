/*
 * @lc app=leetcode id=2220 lang=rust
 *
 * [2220] Minimum Bit Flips to Convert Number
 */

// @lc code=start
impl Solution {
    pub fn min_bit_flips(mut start: i32, mut goal: i32) -> i32 {
        let mut c = 0;
        while start!=0 || goal !=0 {
            if(start % 2 != goal % 2){
                c+=1;
            }
            start = start / 2;
            goal = goal / 2
        }
        c
    }
}
// @lc code=end

