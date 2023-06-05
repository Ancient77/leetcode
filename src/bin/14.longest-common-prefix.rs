/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        while i < strs[0].len() {
            let c = strs[0].get(0..=i).unwrap();
            if strs.iter().clone().all(|x| x.starts_with(c)){
               i+=1;
            }else{
                break;
            }
        }
        
        strs[0].get(0..i).unwrap_or("").to_owned()
    }
}
// @lc code=end

