/*
 * @lc app=leetcode id=1704 lang=rust
 *
 * [1704] Determine if String Halves Are Alike
 */

// @lc code=start
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut firstCount = 0;
        let mut secondCount = 0;
        for (i,c) in s.char_indices(){
            match c.to_ascii_lowercase(){
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    if(i >= s.len() / 2){
                        secondCount+=1;
                    }else{
                        firstCount+=1;
                    }
                }
                _ => ()
            }
        }
        firstCount == secondCount
    }
}
// @lc code=end

