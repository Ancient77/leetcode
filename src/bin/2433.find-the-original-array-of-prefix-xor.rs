/*
 * @lc app=leetcode id=2433 lang=rust
 *
 * [2433] Find The Original Array of Prefix Xor
 */

// @lc code=start
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut resArr = Vec::with_capacity(pref.len());
        for i in 0..pref.len(){
            resArr.push(pref.get(i-1).unwrap_or(&0) ^ pref.get(i).unwrap());
        }
        resArr
    }
}
// @lc code=end

