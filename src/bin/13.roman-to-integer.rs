/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::from([('I',1),('V',5),('X',10),('L',50),('C',100),('D',500),('M',1000)]);
        let mut last = 0;
        let mut counter = 0;
        for n in s.chars().rev(){
            let ele = *map.get(&n).unwrap();
            if ele >= last{
                last = ele;
                counter+=ele;
            }else{
                counter-=ele;
            }
        }
        counter
    }
}
// @lc code=end

