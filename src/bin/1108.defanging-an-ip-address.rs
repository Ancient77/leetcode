/*
 * @lc app=leetcode id=1108 lang=rust
 *
 * [1108] Defanging an IP Address
 */

// @lc code=start
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}
// @lc code=end

