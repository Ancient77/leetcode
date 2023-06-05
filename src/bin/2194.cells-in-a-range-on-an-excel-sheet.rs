/*
 * @lc app=leetcode id=2194 lang=rust
 *
 * [2194] Cells in a Range on an Excel Sheet
 */

// @lc code=start
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut resVec = vec![];
        let mut c = s.chars().collect::<Vec<char>>();
        for n in c[0]..=c[3]{
            for l in c[1]..=c[4]{
                resVec.push(format!("{}{}",n,l));
            }
        }
        resVec
    }
}
// @lc code=end

