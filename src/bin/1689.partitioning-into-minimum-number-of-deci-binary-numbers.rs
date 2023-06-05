/*
 * @lc app=leetcode id=1689 lang=rust
 *
 * [1689] Partitioning Into Minimum Number Of Deci-Binary Numbers
 */

// @lc code=start
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res:i32 = 0;
        for x in n.chars(){
            res = std::cmp::max(res,x.to_digit(10).unwrap() as i32);
            if(res == 9){
                return 9;
            }
        }
        return res;
    }
}
// @lc code=end

