/*
 * @lc app=leetcode id=2169 lang=rust
 *
 * [2169] Count Operations to Obtain Zero
 */

// @lc code=start
impl Solution {
    pub fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
        let mut c = 0;
        while num1 != 0 && num2 != 0 {
            match num1.cmp(&num2){
                 std::cmp::Ordering::Greater | std::cmp::Ordering::Equal=> {
                    num1-=num2;
                 },
                 std::cmp::Ordering::Less => {
                    num2-=num1;
                 }
            }
            c+=1;
        }
        c
    }
}
// @lc code=end

