/*
 * @lc app=leetcode id=1409 lang=rust
 *
 * [1409] Queries on a Permutation With Key
 */



// @lc code=start
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut perm:Vec<i32> = (1..=m).rev().collect();
        let mut res = vec![];
        for x in &queries{
            
            let pos = perm.iter().position(|y| *y == *x).unwrap() as i32;
            
            res.push(perm.len() as i32-pos -1);
            perm.push(*x);
            perm.remove(pos as usize);
        }
        
        res
    }
}
// @lc code=end

