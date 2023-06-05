/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */

// @lc code=start

use std::collections::{BinaryHeap, HashMap};
use std::iter::FromIterator;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|&x| *map.entry(x).or_insert(0)+=1);
        let mut binHeap:BinaryHeap<(&i32,&i32)> = BinaryHeap::from_iter(map.iter().map(|(a,b)| (b,a)));
        let mut res = Vec::with_capacity(k as usize);
        while let Some((_,x)) = binHeap.pop(){
            res.push(*x);
            if(res.len() == k as usize){
                break
            }
        }
        res
        
    }
}
// @lc code=end

