/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */

// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut fib = vec![0,1];
        for i in 2..=n as usize{
            fib.push(fib[i-1] + fib[i-2]);
        }
        fib[n as usize]
    }
}
// @lc code=end

