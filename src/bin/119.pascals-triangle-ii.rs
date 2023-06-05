/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut tria:Vec<Vec<i32>> = vec![];
        for n in 0..=row_index {
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

        return tria.get(row_index as usize).unwrap().to_vec();
    }
}
// @lc code=end

