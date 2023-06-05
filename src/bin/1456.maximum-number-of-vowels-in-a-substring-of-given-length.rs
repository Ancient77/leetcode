/*
 * @lc app=leetcode id=1456 lang=rust
 *
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 */

// @lc code=start
impl Solution {
    pub fn max_vowels(s: String, mut k: i32) -> i32 {
        let sC:Vec<char> = s.chars().collect();
        let mut curScore = 0;
        for x in 0..k as usize{
            if(isVowel(sC[x])){
                curScore+=1;
            }
        }   
        let mut highScore = curScore;
        for i in k as usize..sC.len(){
            if(isVowel(sC[i-k as usize])){
                curScore-=1;
            };
            if(isVowel(sC[i])){
                curScore+=1;
            }
            highScore = highScore.max(curScore)
        }
        return highScore;
    }
}
fn isVowel(i:char)->bool {
    (i == 'a' ||i == 'e' ||i == 'i' ||i == 'o' ||i == 'u' )
}
// @lc code=end

