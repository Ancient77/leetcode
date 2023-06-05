/*
 * @lc app=leetcode id=1588 lang=rust
 *
 * [1588] Sum of All Odd Length Subarrays
 */

// @lc code=start
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut i = 1;
        let mut count: i32 = 0;
        while i<=arr.len() {
            for window in arr.windows(i){
                count+=window.iter().sum::<i32>();
            }
            i+=2;
        }
        count
    }
}
// @lc code=end

