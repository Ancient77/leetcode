/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut s = 0;
        while (x - s*s) >= 0{
            s+=1;
        }
        s - 1 
    }
}
// @lc code=end

