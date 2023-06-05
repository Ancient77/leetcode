/*
 * @lc app=leetcode id=1967 lang=rust
 *
 * [1967] Number of Strings That Appear as Substrings in Word
 */

// @lc code=start
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut sum = 0;
        for n in patterns{
            if(word.contains(&n)){
                sum+=1;
            }
        }
        sum
    }
}
// @lc code=end

