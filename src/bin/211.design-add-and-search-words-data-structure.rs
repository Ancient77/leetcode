/*
 * @lc app=leetcode id=211 lang=rust
 *
 * [211] Design Add and Search Words Data Structure
 */





// @lc code=start
use std::{collections::HashMap, str::Chars};
struct WordDictionary {
    children:HashMap<char,WordDictionary>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self { children:HashMap::new() }
    }
    
    fn add_word(&mut self, mut word: String) {
        word.push_str("ö");
       let mut iter = word.chars();
        add_iter(self,&mut iter);
    }
    
    fn search(&self, mut word: String) -> bool {
        word.push_str("ö");
        let mut iter = word.chars();
        search_iter(&self,&mut iter)
    }

    
}
fn add_iter(trie:&mut WordDictionary,chars:&mut Chars){
    match chars.next() {
        None => (),
        Some(x) => {
            match trie.children.get_mut(&x) {
                None => {
                    trie.children.insert(x, WordDictionary::new());
                    add_iter(trie.children.get_mut(&x).unwrap(), chars)
                },
                Some( mut n) => {add_iter( &mut n, chars);}
            }
        }
    }
}
fn search_iter(trie:&WordDictionary,mut chars:&mut Chars)->bool{
    match chars.next() {
        Some('.') => trie.children.iter().any(|(char,wordDic)| search_iter(wordDic, &mut chars.clone())),
        Some(x) => match trie.children.get(&x){
            Some(y) =>  search_iter(y, &mut chars.clone()),
            None => return false
        },
        None => return true

    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
// @lc code=end

