/*
 * @lc app=leetcode id=1281 lang=rust
 *
 * [1281] Subtract the Product and Sum of Digits of an Integer
 */

use std::ops::Mul;

// @lc code=start
impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut digits = Vec::new();
        while n>0 {
            digits.push(n%10);
            n/=10;
        }
        digits.reverse();
        digits.iter().product::<i32>() - digits.iter().sum::<i32>()

    }
}
// @lc code=end

