/*
 * @lc app=leetcode id=1282 lang=rust
 *
 * [1282] Group the People Given the Group Size They Belong To
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..group_sizes.len(){
            match map.get_mut(&group_sizes[i]) {
                Some(x) => x.push(i as i32),
                None => {map.insert(group_sizes[i], vec![i as i32]);},
            }
        }
        let mut resVec: Vec<Vec<i32>> = vec![];
        for (i,group) in map{
            resVec.append(&mut group.chunks(i as usize).map(|x: &[i32]| x.to_vec()).collect());
        }
        resVec
    }
}
// @lc code=end

