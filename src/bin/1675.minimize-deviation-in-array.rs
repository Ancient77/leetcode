/*
 * @lc app=leetcode id=1675 lang=rust
 *
 * [1675] Minimize Deviation in Array
 */

// @lc code=start

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        
        let mut min = if (nums[0] % 2 == 0){nums[0]}else{nums[0]*2};
        nums.remove(0);
        for n in &nums {
            if *n >= min {
                break;
            }
            if *n % 2 == 0 {
                min = *n;
                break;
            }
            min = *n;
        }
        let mut binHeap = BinaryHeap::from(nums);
        loop {
            let x = binHeap.pop().unwrap();
            if x % 2 == 0 {
                binHeap.push(x / 2)
            } else {
                return x - min;
            }
        }
    }
}
// @lc code=end
