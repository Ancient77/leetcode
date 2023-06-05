/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut tria:Vec<Vec<i32>> = vec![];
        for n in 0..num_rows {
            
            if(n==0){
                tria.push(vec![1]);
            }else if (n == 1){
                tria.push(vec![1,1])
            }
            else{
                let mut tempVec:Vec<i32> = vec![];
                for s in 0..=n {
                    tempVec.push((tria[(n-1) as usize].get((s-1) as usize).unwrap_or(&0)) + (tria[(n-1) as usize].get(s as usize).unwrap_or(&0)))
                }
                tria.push(tempVec)
            }
            
        }
        tria
    }
}
// @lc code=end

