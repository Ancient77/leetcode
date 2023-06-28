/*
 * @lc app=leetcode id=1514 lang=rust
 *
 * [1514] Path with Maximum Probability
 */


// @lc code=start

use std::collections::BinaryHeap;
use core::cmp::Ordering;

#[derive(PartialEq)]
struct Tupel(f64,i32);

impl Eq for Tupel {}

impl PartialOrd for Tupel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Tupel {
    fn cmp(&self, other: &Tupel) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start: i32, end: i32) -> f64 {
        let mut maxProbs = vec![0.0;n as usize];
        maxProbs[start as usize] = 1.0;
        let mut graph: Vec<Vec<(i32,f64)>> = vec![vec![];n as usize];
        for (i,edge) in edges.iter().enumerate(){
            graph[edge[0] as usize].push((edge[1], succ_prob[i]));
            graph[edge[1] as usize].push((edge[0], succ_prob[i]));
        }
        let mut pq: BinaryHeap<Tupel> = BinaryHeap::new();
        pq.push(Tupel(1f64,start));
        while !pq.is_empty() {
            let Tupel(prob,node) = pq.pop().unwrap();
            if(node == end){
                return prob;
            }
            for (nextNode, nextProb) in &graph[node as usize]{
                if(maxProbs[*nextNode as usize] < *nextProb  * prob){
                    maxProbs[*nextNode as usize] = *nextProb * prob;
                    pq.push(Tupel(*nextProb * prob, *nextNode))
                }
            }
        }

        return 0f64;
    }
}
// @lc code=end

