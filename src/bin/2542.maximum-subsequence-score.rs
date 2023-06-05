/*
 * @lc app=leetcode id=2542 lang=rust
 *
 * [2542] Maximum Subsequence Score
 */


// @lc code=start
use std::{cmp::Reverse};
impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let mut pairs: Vec<(i64,i64)> = Vec::<(i64,i64)>::with_capacity(nums1.len());
        for x in 0..nums1.len(){
            pairs.push((nums2[x] as i64,nums1[x] as i64));
        }
        pairs.sort_unstable();
        let mut pairIter = pairs.iter().rev();
        let mut pq = std::collections::BinaryHeap::<Reverse<i64>>::with_capacity(k as usize);
        let mut sum = 0;
        let mut res = 0;
        for i in 0..k{
            let x = *pairIter.next().unwrap();
            sum+=x.1;
            res=x.0;
            pq.push(Reverse(x.1));
        }
        res= res*sum;

        while let Some(&x) = pairIter.next() {
            sum-=pq.pop().unwrap().0;
            sum+=x.1;
            pq.push(Reverse(x.1));

            res = res.max(sum*x.0);
        }
        res
    }
}
// @lc code=end

