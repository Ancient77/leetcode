/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //TIME LIMIT EXCEEDED 
        //NEED TO REWRITE WITH BINARY SEARCH FROM THAT POINT ON
        
        let mut res = vec![];
        for x in 0..nums.len() {
            let num1 = nums[x];
            for y in x..nums.len() {
                let num2 = nums[y];
                if(x == y){continue;}
                for z in y..nums.len() {
                    
                    let num3 = nums[z];
                    if(z == y || z ==x ){continue;}
                    if(num1 + num2 + num3 == 0){
                        let mut temp = vec![num1,num2,num3];
                        temp.sort_unstable();
                        res.push(temp)
                    }
                }
            }
            
        }
        res.sort();
        res.dedup();
        res
    }
}
// @lc code=end
