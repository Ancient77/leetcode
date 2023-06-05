/*
 * @lc app=leetcode id=837 lang=rust
 *
 * [837] New 21 Game
 */

// @lc code=start
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let k = k as usize;
        let maxf64 = max_pts as f64;
        let max_pts = max_pts as usize;
        let mut dp = vec![0.0; k + max_pts];
        dp[0] = 1.0;
        if dp.len() > 1 {
            dp[1] = -1.0;
        }
        let bound = (n as usize + 1).min(k + max_pts);
        for i in 1..bound {
            dp[i] += dp[i - 1];
            if i <= k {
                dp[i] += dp[i - 1] / maxf64;
            }
            if i > max_pts {
                dp[i] -= dp[i - max_pts - 1] / maxf64;
            }
        }
        dp[k..bound].iter().sum::<f64>()
    }
}
// @lc code=end