/*
 * @lc app=leetcode id=2545 lang=rust
 *
 * [2545] Sort the Students by Their Kth Score
 */

// @lc code=start
impl Solution {
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_by_key(|x| -x[k as usize]);
        score
    }
}
// @lc code=end

