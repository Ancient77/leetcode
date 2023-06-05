/*
 * @lc app=leetcode id=2140 lang=rust
 *
 * [2140] Solving Questions With Brainpower
 */

// @lc code=start
impl Solution {
    pub fn most_points(mut questions: Vec<Vec<i32>>) -> i64 {
        let mut max: Vec<i64> = Vec::with_capacity(questions.len());
        
        max.push(questions[questions.len()-1][0] as i64);
        for i in 1..questions.len(){
            //if you take
            let one:i64 = *max.get(i-questions[questions.len()-i-1][1] as usize-1).unwrap_or(&0) + questions[questions.len()-i-1][0] as i64;
            //if you dont take
            let two = *max.get(i-1).unwrap();
            max.push(one.max(two));
        }
        *max.last().unwrap() as i64
    }
}
// @lc code=end

