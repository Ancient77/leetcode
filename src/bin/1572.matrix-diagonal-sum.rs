/*
 * @lc app=leetcode id=1572 lang=rust
 *
 * [1572] Matrix Diagonal Sum
 */

// @lc code=start
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        for i in 0..mat.len(){
            sum+=mat[i][i];
            sum+=mat[i][mat.len()-1-i];
        }
        if(mat.len() % 2 == 1 && mat[0].len() %2 == 1){
            sum-=mat[mat.len()/2][mat.len()/2];
        }
        sum
    }
}
// @lc code=end

