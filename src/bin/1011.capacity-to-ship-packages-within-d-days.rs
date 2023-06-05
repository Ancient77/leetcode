/*
 * @lc app=leetcode id=1011 lang=rust
 *
 * [1011] Capacity To Ship Packages Within D Days
 */

// @lc code=start
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut cur:i32 = (weights.iter().sum::<i32>() / days);
        loop {
            let mut coWeights = weights.clone();
            coWeights.reverse();
            
            for n in 0..days{
                let mut left = cur;
                while !coWeights.is_empty() && left >= *coWeights.last().unwrap(){
                    left-=*coWeights.last().unwrap();
                    coWeights.pop();
                }
            }
            if(coWeights.is_empty()){return cur;}
            cur+=1;
        }
    }
}
// @lc code=end

