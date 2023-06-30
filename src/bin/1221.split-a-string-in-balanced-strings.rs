/*
 * @lc app=leetcode id=1221 lang=rust
 *
 * [1221] Split a String in Balanced Strings
 */

// @lc code=start
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut p = 0;
        let mut res = 0;
        for c in s.chars(){
            match c{
                'L' => p+=1,
                'R' => p-=1,
                _ => ()
            }
            if(p == 0){
                res+=1;
            }
        }
        res
    }
}
// @lc code=end

