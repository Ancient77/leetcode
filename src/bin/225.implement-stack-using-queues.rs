/*
 * @lc app=leetcode id=225 lang=rust
 *
 * [225] Implement Stack using Queues
 */

// @lc code=start
struct MyStack {
    vec:Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack { vec: Vec::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.vec.push(x)
    }
    fn pop(&mut self) -> i32 {
        self.vec.pop().unwrap()
    }
    
    fn top(&self) -> i32 {
        self.vec[self.vec.len()-1]
    }
    
    fn empty(&self) -> bool {
        self.vec.is_empty()
    }
}


// @lc code=end

