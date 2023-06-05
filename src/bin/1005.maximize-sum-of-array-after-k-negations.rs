/*
 * @lc app=leetcode id=1005 lang=rust
 *
 * [1005] Maximize Sum Of Array After K Negations
 */



// @lc code=start
impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        for n in 0..k{
            let num = -nums.last().unwrap();
            nums.pop();
            nums.insert(nums.binary_search_by(|&x| num.cmp(&x)).unwrap_or_else(|err| err), num);
        }
        nums.iter().sum()
    }
}
// @lc code=end

