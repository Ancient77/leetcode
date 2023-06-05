/*
 * @lc app=leetcode id=1822 lang=rust
 *
 * [1822] Sign of the Product of an Array
 */

// @lc code=start
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        for n in nums{
            match n.cmp(&0) {
                std::cmp::Ordering::Less => res*=-1,
                std::cmp::Ordering::Greater => res*=1,
                std::cmp::Ordering::Equal => return 0
            };

        }
        res
    }
}
// @lc code=end

