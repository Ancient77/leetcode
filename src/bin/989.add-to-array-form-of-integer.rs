/*
 * @lc app=leetcode id=989 lang=rust
 *
 * [989] Add to Array-Form of Integer
 */

// @lc code=start
impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
       num.reverse();
       let mut n = 0;
       while k != 0 {
            let val = k % 10;
            k = k / 10;
            if(n >= num.len()){
                num.push(0)
            }
            if(val + num[n] > 9){
                k+=1;
                num[n]+= val - 10;
            }else{
                num[n]+= val;
            }
            n+=1;
       }
       num.reverse();
       num
    }
}
// @lc code=end

