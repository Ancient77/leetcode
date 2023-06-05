/*
 * @lc app=leetcode id=2325 lang=rust
 *
 * [2325] Decode the Message
 */

// @lc code=start

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut set: HashSet<char> = HashSet::with_capacity(key.len());
        let mut map: HashMap<char, char> = HashMap::from_iter(key.chars().filter(|x| *x != ' ' && set.insert(*x)).zip(('a'..='z').into_iter()));
        let mut res: String = String::with_capacity(message.len());
        for s in message.chars(){
            if(s == ' '){
                res.push(s);
            }else{
                res.push(*map.get(&s).unwrap());
            }
        }
        res   
    }
}
// @lc code=end

