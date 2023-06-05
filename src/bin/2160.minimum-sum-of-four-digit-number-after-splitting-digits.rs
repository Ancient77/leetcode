/*
 * @lc app=leetcode id=2160 lang=rust
 *
 * [2160] Minimum Sum of Four Digit Number After Splitting Digits
 */

// @lc code=start
impl Solution {
    pub fn minimum_sum(mut num: i32) -> i32 {
        let mut numVec = Vec::with_capacity(4);
        while num > 0 {
            numVec.push(num%10);
            num /= 10;
        }
        numVec.sort_unstable();
        numVec[3] + numVec[2] + numVec[1]*10 + numVec[0]*10
    }
}
// @lc code=end

