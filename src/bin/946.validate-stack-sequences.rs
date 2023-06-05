/*
 * @lc app=leetcode id=946 lang=rust
 *
 * [946] Validate Stack Sequences
 */

// @lc code=start
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut pointerOne = 0;
        let mut pointerTwo = 1;
        stack.push(pushed[0]);
        while pointerOne < popped.len() -1 {
            
            
            if(*stack.last().unwrap_or(&-1) == popped[pointerOne]){
                stack.pop();
                pointerOne+=1;
            }else{
                if(pointerTwo > popped.len()-1){
                    return false;
                }
                stack.push(pushed[pointerTwo]);
                pointerTwo+=1;
            }
        }
        true
    }
}
// @lc code=end


