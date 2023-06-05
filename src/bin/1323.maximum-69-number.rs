/*
 * @lc app=leetcode id=1323 lang=rust
 *
 * [1323] Maximum 69 Number
 */

// @lc code=start
impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut temp = num;
        let mut i = 0;
        let mut iLast:i32 = -1;
        while temp > 0 {
            if temp % 10 == 6{
                iLast = i as i32;
                i+=1;
                temp /=10
            }else{
                temp /=10;
                i+=1;
            }
        }
        if(iLast == -1){
            return num
        }
        return num + 3*10_i32.pow(iLast as u32);
    }
}
// @lc code=end

