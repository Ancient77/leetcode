/*
 * @lc app=leetcode id=2022 lang=rust
 *
 * [2022] Convert 1D Array Into 2D Array
 */

// @lc code=start
impl Solution {
    pub fn construct2_d_array(mut original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for x in original.chunks(n as usize){
            res.push(x.to_owned());
        }
        if(res.len() != m as usize ){return vec![];}
        res
    }
}
// @lc code=end

