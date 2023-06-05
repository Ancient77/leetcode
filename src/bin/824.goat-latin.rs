/*
 * @lc app=leetcode id=824 lang=rust
 *
 * [824] Goat Latin
 */

// @lc code=start
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let words:Vec<&str> = sentence.split(' ').collect();
        let res = Vec::new();
        for n in 0..words.len() {
            if(words[n].starts_with(|x| x== a | e | i | o | u)){
                words[n] = words[n] + "ma";
            }else{
                words[n].
            }
            words[n].replace(from, to)
        }

    }
}
// @lc code=end

