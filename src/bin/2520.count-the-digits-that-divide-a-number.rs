/*
 * @lc app=leetcode id=2520 lang=rust
 *
 * [2520] Count the Digits That Divide a Number
 */

// @lc code=start
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut count = 0;
        let mut num2 = num;
        while num2 > 0 {
            if(num % (num2%10) == 0){
                count+=1;
            }
            num2/=10;
        }
        count
    }
}
// @lc code=end

