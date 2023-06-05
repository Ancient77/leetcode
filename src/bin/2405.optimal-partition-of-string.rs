/*
 * @lc app=leetcode id=2405 lang=rust
 *
 * [2405] Optimal Partition of String
 */

// @lc code=start
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut cache:Vec<char> = vec![];
        let mut sum = if s.len() > 0 {1}else{0};
        for n in s.chars(){
            if(cache.contains(&n)){
                sum+=1;
                cache.clear();
            }
            cache.push(n);
        }
        sum
    }
}
// @lc code=end

