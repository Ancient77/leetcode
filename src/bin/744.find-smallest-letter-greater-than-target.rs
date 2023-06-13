/*
 * @lc app=leetcode id=744 lang=rust
 *
 * [744] Find Smallest Letter Greater Than Target
 */


// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let temp = letters.binary_search_by(|x|{
            if(*x > target){
                return Ordering::Greater;
            }
            if(*x <= target){
                return Ordering::Less;
            }
            
            panic!()
        }).unwrap_or_else(|y| y-1 )+1;
        
        if(temp == letters.len()){
            return letters[0];
        }
        
        letters[temp]
    }
}
// @lc code=end

