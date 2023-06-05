/*
 * @lc app=leetcode id=208 lang=rust
 *
 * [208] Implement Trie (Prefix Tree)
 */



// @lc code=start
use std::collections::HashSet;
struct Trie {
    set:HashSet<String>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    
    fn new() -> Self {
        Self { set: HashSet::new() }
    }
    
    fn insert(& mut self, word: String) {
        self.set.insert(word);
    }
    
    fn search(&self, word: String) -> bool {
        self.set.contains(&word)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        for word in &self.set{
            if(word.starts_with(&prefix)){
                return true;
            }
        }
        return false;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

