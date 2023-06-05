/*
 * @lc app=leetcode id=2011 lang=rust
 *
 * [2011] Final Value of Variable After Performing Operations
 */

// @lc code=start
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0;
        for x in operations{
            if(x.contains("+")){
                res += 1;
            }else if x.contains("-") {
                res -= 1;
            }
        }
        res
    }
}
// @lc code=end

