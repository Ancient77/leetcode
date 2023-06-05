/*
 * @lc app=leetcode id=1374 lang=rust
 *
 * [1374] Generate a String With Characters That Have Odd Counts
 */

// @lc code=start
impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut res:String = "x".to_owned();
        
        if(n % 2 == 0){
            
            res = res.repeat(n as usize-1);
            res.push('y');
        }else{
            res = res.repeat(n as usize);
            
        }
        res
    }
}
// @lc code=end

