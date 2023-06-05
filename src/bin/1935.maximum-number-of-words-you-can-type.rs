/*
 * @lc app=leetcode id=1935 lang=rust
 *
 * [1935] Maximum Number of Words You Can Type
 */



// @lc code=start
use std::str::Chars;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut sum = 0;
        let words = text.split(' ');
        let letters = broken_letters.chars();
        for n in words{
            if(!contains(n,&letters)){
                sum+=1;
            }
        }
        return sum;
    }
    
}
fn contains(word:&str,letters:&Chars) -> bool{
    for n in letters.to_owned(){
        if(word.contains(n)){
            return true;
        }
    }
    return false;
}
// @lc code=end

