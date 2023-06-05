/*
 * @lc app=leetcode id=2206 lang=rust
 *
 * [2206] Divide Array Into Equal Pairs
 */


// @lc code=start
use std::collections:: HashSet;
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut occ: HashSet<i32> = HashSet::new();
        for num in nums{
            match occ.contains(&num){
                true => {occ.remove(&num)},
                false => {occ.insert(num)}
            };
        }
        return occ.is_empty();
    }
}
// @lc code=end

