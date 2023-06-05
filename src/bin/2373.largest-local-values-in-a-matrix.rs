/*
 * @lc app=leetcode id=2373 lang=rust
 *
 * [2373] Largest Local Values in a Matrix
 */

// @lc code=start
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut resVec = vec![vec![0;grid.len()-2];grid.len()-2];
        for i in 1..grid.len()-1{
            for j in 1..grid.len()-1{
                resVec[i-1][j-1] = *vec![grid[i-1][j-1],grid[i-1][j],grid[i-1][j+1],grid[i][j-1],grid[i][j],grid[i][j+1],grid[i+1][j-1],grid[i+1][j],grid[i+1][j+1]].iter().max().unwrap();
            }
        }
        resVec
    }
}
// @lc code=end

