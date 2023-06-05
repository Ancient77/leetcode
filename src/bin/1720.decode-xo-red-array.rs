/*
 * @lc app=leetcode id=1720 lang=rust
 *
 * [1720] Decode XORed Array
 */

// @lc code=start
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut resVec = Vec::with_capacity(encoded.len());
        resVec.push(first);
        for n in 0..encoded.len(){
            resVec.push(encoded[n]^resVec[n]);
        }
        resVec
    }
}
// @lc code=end

