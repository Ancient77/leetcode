/*
 * @lc app=leetcode id=771 lang=rust
 *
 * [771] Jewels and Stones
 */

// @lc code=start
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut count = 0;
        for n in stones.chars(){
            if(jewels.contains(n)){
                count+=1;
            }
        }
        count
    }
}
// @lc code=end

