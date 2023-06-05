/*
 * @lc app=leetcode id=1046 lang=rust
 *
 * [1046] Last Stone Weight
 */

// @lc code=start

//Ok imma be honest. This is a really bad solution.
// I was in Berufsschule at the time and didnt want to do it better
impl Solution {
    pub fn last_stone_weight(mut stones: Vec<i32>) -> i32 {
        stones.sort();
        while stones.len() > 1 {
            if(stones[stones.len() - 1] == stones[stones.len() - 2]){
                stones.pop();
                stones.pop();
            }else{
                let temp  = stones[stones.len() - 1] - stones[stones.len() - 2];
                stones.pop();
                stones.pop();
                match stones.binary_search(&temp){
                    Ok(pos) => stones.insert(pos, temp),
                    Err(pos) => stones.insert(pos, temp)
                }
                
            }
        }
        if(stones.len() == 0){
            return 0;
        }
        return stones[0];
    }
}
// @lc code=end

