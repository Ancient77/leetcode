/*
 * @lc app=leetcode id=502 lang=rust
 *
 * [502] IPO
 */



// @lc code=start

use std::collections::BinaryHeap;


impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let len = profits.len();
        let mut projects = Vec::new();
        
        for i in 0..len{
            projects.push((profits[i],capital[i]));
        }
        projects.sort_unstable_by_key(|x| (*x).1);
        let mut money = w;
        let mut possbileProjects = BinaryHeap::new();
        let mut i:usize = 0;
        for n in 0..k as usize{
            while i<len && projects[i].1 <= money {
                possbileProjects.push(projects[i]);
                i+=1;
            }
            if let Some(x) = possbileProjects.pop() {
                money+= x.0;
            }else{
                break;
            }
        }
        money

    }
}
// @lc code=end

