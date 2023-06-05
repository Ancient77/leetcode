/*
 * @lc app=leetcode id=1688 lang=rust
 *
 * [1688] Count of Matches in Tournament
 */

// @lc code=start
impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut matchCount = 0;
        while n != 1 {
            match n % 2 == 0{
                false => {
                    matchCount+=(n-1)/2;
                    n = (n+1)/2;
                },
                true => {
                    matchCount+=n/2;
                    n= n/2;
                }
            }
        }

        matchCount
    }
}
// @lc code=end

