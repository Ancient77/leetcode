/*
 * @lc app=leetcode id=703 lang=rust
 *
 * [703] Kth Largest Element in a Stream
 */

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

// @lc code=start

use std::collections::BinaryHeap;
use std::iter::FromIterator;


struct KthLargest {
    prioMinHeap: BinaryHeap<i32>,
    
    k:i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        
        nums.sort_unstable();
        let mut x = BinaryHeap::from_iter(nums.iter().rev().map(|y| -*y).take(k as usize));
        
        Self { prioMinHeap: x, k,}

    }
    
    fn add(&mut self, val: i32) -> i32 {
        if(self.prioMinHeap.len() < self.k as usize){
            self.prioMinHeap.push(-val);
            return -*self.prioMinHeap.peek().unwrap();
        }
        match self.prioMinHeap.peek().unwrap().cmp(&-val){
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Less=> (),
            std::cmp::Ordering::Greater => {
                self.prioMinHeap.pop();
                self.prioMinHeap.push(-val);
            }

        }
        
        -*self.prioMinHeap.peek().unwrap()
    }
}
// @lc code=end

