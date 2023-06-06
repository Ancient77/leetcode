/*
 * @lc app=leetcode id=1502 lang=rust
 *
 * [1502] Can Make Arithmetic Progression From Sequence
 */

// @lc code=start
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let diff = arr[0] - arr[1];
        arr.windows(2).all(|x| x[0] - x[1] == diff)
    }
}
// @lc code=end

