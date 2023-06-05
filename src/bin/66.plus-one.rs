/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        digits.reverse();
        for i in 0..digits.len() {
            if(digits[i] != 9){
                digits[i] += 1;
                digits.reverse();
                return digits;
            }
            digits[i] = 0;
        }
        digits.push(1);
        digits.reverse();
        digits
    }
}
// @lc code=end
