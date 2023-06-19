/*
 * @lc app=leetcode id=1732 lang=rust
 *
 * [1732] Find the Highest Altitude
 */

// @lc code=start
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest = 0;
        let mut cur = 0;
        for x in gain{
            cur +=x;
            if(cur > highest){
                highest = cur;
            }
        }
        highest
    }
}
// @lc code=end

