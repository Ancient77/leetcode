/*
 * @lc app=leetcode id=1351 lang=rust
 *
 * [1351] Count Negative Numbers in a Sorted Matrix
 */

// @lc code=start
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter().fold(0,|acc, x| acc+ (x.iter().filter(|y| **y<0).count() as i32))
    }
}
// @lc code=end

