/*
 * @lc app=leetcode id=2390 lang=rust
 *
 * [2390] Removing Stars From a String
 */

// @lc code=start
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut charIter = s.chars().rev();
        let mut res = Vec::with_capacity(100);
        let mut c = 0;
        for x in charIter{
            if(x == '*'){
                c+=1;
            }else{
                if(c > 0){
                    c-=1;
                }else{
                    res.push(x);
                }
            }
        }
        res.iter().rev().collect()
    }
}
// @lc code=end

