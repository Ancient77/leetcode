/*
 * @lc app=leetcode id=2090 lang=rust
 *
 * [2090] K Radius Subarray Averages
 */

// @lc code=start
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if(k as usize*2 >= nums.len()){
            return vec![-1;nums.len()];
        }
        



        println!("{k}");
        println!("{:?}",nums.len());
        
        
        let mut res: Vec<i32> = Vec::with_capacity(nums.len());
        let divider: i128 = (2*k+1) as i128 ;
        for i in 0..k {
            res.push(-1);
        }
        let mut sum:i128 = nums.iter().take(divider as usize).map(|x| *x as i128).sum::<i128>();

        res.push((sum/divider) as i32);
        for i in k as usize..nums.len()-k as usize-1{
            sum-= nums[i-k as usize] as i128;
            sum+= nums[i+k as usize+1] as i128;

            res.push((sum/divider) as i32);
        }


        for i in 0..k {
            res.push(-1);
        }
        res
    }
}
// @lc code=end

