/*
 * @lc app=leetcode id=2315 lang=rust
 *
 * [2315] Count Asterisks
 */

// @lc code=start
impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut chunks = s.split('|');
        let mut count: i32 = 0;
        
        while let Some(chunk) = chunks.next() {
            count+= chunk.chars().filter(|&x| x == '*').count() as i32;
            chunks.next();
        }
        count
    }
}
// @lc code=end

