/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue {
    vec:Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue { vec: Vec::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.vec.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        let temp = self.vec[0];
        for n in 0..self.vec.len()-1{
            self.vec[n] = self.vec[n+1];
        }
        self.vec.pop();
        temp
    }
    
    fn peek(&self) -> i32 {
        self.vec[0]
    }
    
    fn empty(&self) -> bool {
        self.vec.is_empty()
    }
}


// @lc code=end

