/*
 * @lc app=leetcode id=1304 lang=rust
 *
 * [1304] Find N Unique Integers Sum up to Zero
 */

// @lc code=start
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(n as usize);
        for i in 1..n/2+1{
            res.push(i*-1);
            res.push(i);
        }
        if(n % 2 == 1){
            res.push(0);
        }
        res
    }
}
// @lc code=end

