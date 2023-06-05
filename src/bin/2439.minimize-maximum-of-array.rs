/*
 * @lc app=leetcode id=2439 lang=rust
 *
 * [2439] Minimize Maximum of Array
 */

// @lc code=start
impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut max:f64 = 0.0;
        let mut tempSum:f64 = 0.0;
        for x in 0..nums.len(){
            tempSum+= nums[x] as f64;
            if(tempSum / (x as f64 + 1.0) > max){
                max = tempSum / (x as f64 + 1.0);
            }
        }
        print!("{:?}",max);
        max.ceil() as i32 
    }
}
// @lc code=end

