/*
 * @lc app=leetcode id=2610 lang=rust
 *
 * [2610] Convert an Array Into a 2D Array With Conditions
 */

// @lc code=start
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut index: Vec<usize> = vec![0; nums.len()];
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(nums.len());
        for x in nums{
            match res.get_mut(index[x as usize - 1]){
                Some(mut y) => y.push(x),
                None => res.push(vec![x]) 
            }
            index[x as usize-1] += 1;
            
        }
        return res
    }
}
// @lc code=end

