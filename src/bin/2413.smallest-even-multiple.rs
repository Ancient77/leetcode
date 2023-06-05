/*
 * @lc app=leetcode id=2413 lang=rust
 *
 * [2413] Smallest Even Multiple
 */

// @lc code=start
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if(n%2==0){
            return n;
        }else{
            return n*2;
        }
    }
}
// @lc code=end

