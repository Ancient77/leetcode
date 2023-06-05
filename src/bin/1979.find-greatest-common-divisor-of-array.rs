/*
 * @lc app=leetcode id=1979 lang=rust
 *
 * [1979] Find Greatest Common Divisor of Array
 */

// @lc code=start
impl Solution {
    pub fn find_gcd(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let low = nums[0];
        let high = *nums.last().unwrap();
        for i in (0..=low).rev() {
            if(low % i == 0 && high % i == 0){
                return i;
            }
        }
        1
    }
}
// @lc code=end

