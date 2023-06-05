/*
 * @lc app=leetcode id=87 lang=rust
 *
 * [87] Scramble String
 */



// @lc code=start

use std::{cmp::Ordering, collections::HashMap};



struct Node{
    next: HashMap<char,Node>,
    done:bool,
    isEnd:bool
}
impl Node {
    fn new() -> Self{
        Self { 
            next: HashMap::new(),
            done: false,
            isEnd:false
         }
        
    }
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let mut pointer = Node::new();
        let startNode = pointer;
        for s in s2.chars(){
            let temp = Node::new();
            pointer.next.insert(s, temp);
            pointer = temp;
        }
        pointer.isEnd = true;
        

    }
}

fn rec(cur:Vec<char>,endRef:&Vec<char>)-> bool {
    
    
}

fn check(cur:&Vec<char>,end:&Vec<char>)-> bool {
    
    let mut temp = cur.clone();
    temp.retain(|x| x.is_alphabetic());
    let mut iter1 = temp.iter();
    let mut iter2 = end.iter();
    if(iter1.len() != iter2.len()){return false;}
    while iter2.len() != 0 {
        if(iter1.next() != iter2.next()){return false;}
    }
    true
}
// @lc code=end

