/*
 * @lc app=leetcode id=807 lang=rust
 *
 * [807] Max Increase to Keep City Skyline
 */


// @lc code=start
impl Solution {
    
    
    //this creates a new array and goes through each row and sets the max value for each
    //aka it only need to calculate the max n+n times
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut resArr = vec![vec![std::i32::MAX;grid.len()];grid.len()];
        for i in 0..grid.len(){
            let max = *grid[i].iter().max().unwrap();
            for j in 0..grid.len(){
                resArr[i][j] = resArr[i][j].min(max);
            }
        }
        for i in 0..grid.len(){
            let mut max =0;
            for j in 0..grid.len(){
                max = max.max(grid[j][i])
            }
            for j in 0..grid.len(){
                resArr[j][i] = resArr[j][i].min(max);
            }
        }
        let mut c = 0;
        for i in 0..grid.len(){
            for j in 0..grid.len(){
                c += resArr[i][j] - grid[i][j];
            }
        }
        c
    }
    
    
    //this goes does not create a new vec
    // this needs to calc the highest n+n*n
    //time wise they are both 0ms
    pub fn first_try_max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut c = 0;
        for i in 0..grid.len(){
            let ele = &grid[i];
            let mut highest = *ele.iter().max().unwrap();
            for j in 0..ele.len(){
                let mut columnHighest = 0;
                for n in 0..ele.len(){
                    columnHighest = columnHighest.max(grid[n][j]);
                }
                c+=columnHighest.min(highest) - grid[i][j];
            }
        }
        c
    }
}
// @lc code=end

