/*
 * @lc app=leetcode id=1769 lang=rust
 *
 * [1769] Minimum Number of Operations to Move All Balls to Each Box
 */

// @lc code=start
impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut res = vec![0;boxes.len()];
        let vecBoxes = boxes.chars().map(|x| x.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
        let mut howMany = (vecBoxes[0],vecBoxes.iter().sum::<i32>());
        for i in 1..boxes.len(){
            res[0]+=vecBoxes[i];
        }
        for i in 0..boxes.len(){
            res[i] = howMany;
            vecBoxes[i]
        }
        res
    }
}
// @lc code=end

