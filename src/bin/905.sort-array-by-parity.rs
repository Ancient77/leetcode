/*
 * @lc app=leetcode id=905 lang=rust
 *
 * [905] Sort Array By Parity
 */


// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by(|a,b| {
            match (a % 2, b % 2){
                (0,0) => return Ordering::Equal,
                (1,0) => return Ordering::Greater,
                (0,1) => return Ordering::Less,
                (1,1) => return Ordering::Equal,
                _ => panic!()
            }
        });
        nums
    }
}
// @lc code=end

