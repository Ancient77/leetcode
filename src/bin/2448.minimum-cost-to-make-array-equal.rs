/*
 * @lc app=leetcode id=2448 lang=rust
 *
 * [2448] Minimum Cost to Make Array Equal
 */

// @lc code=start
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let mut x:Vec<(usize,usize)> = nums.iter().zip(cost.iter()).map(|(&x,&y)|(x as usize,y as usize)).collect();
        x.sort_unstable();
        let mut left_cost = 0;
        let mut right_cost: usize = x.iter().map(|(v, c)| c).sum::<usize>();
        let mut last = (0,0);
        let mut cur_total = x.iter().fold(0, |acc,(height,cost)| acc+cost*height);
        let mut min_cost: usize = cur_total;
        for (height,cost) in x.iter(){
            
            left_cost+=last.1;
            cur_total+=left_cost*(*height-last.0);
            cur_total-= right_cost*(*height-last.0);
            right_cost-=*cost;
            last = (*height,*cost);
            min_cost = min_cost.min(cur_total);
        }
        min_cost as i64
    }
}
// @lc code=end

