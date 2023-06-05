/*
 * @lc app=leetcode id=748 lang=rust
 *
 * [748] Shortest Completing Word
 */

// @lc code=start
impl Solution {
    pub fn shortest_completing_word(mut license_plate: String, words: Vec<String>) -> String {
        let mut wordsCopy = words.clone();
        let mut license_plate2:Vec<char> = license_plate.chars().filter(|c| c.is_alphabetic()).map(|x| x.to_ascii_lowercase()).collect();
        let mut returnVec = vec![];
        let mut i = 0;
        'outer:for mut word in wordsCopy{
            for n in &license_plate2{
                match word.find(*n){
                    Some(x) => {word.remove(x);},
                    None => {i+=1;continue 'outer}
                }
                
            }
            returnVec.push(words[i].clone());
            i+=1;
        }
        returnVec.sort_by(|a, b| a.len().cmp(&b.len()));
        return returnVec[0].clone();
    }
}
// @lc code=end

