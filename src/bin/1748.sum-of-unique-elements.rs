/*
 * @lc app=leetcode id=1748 lang=rust
 *
 * [1748] Sum of Unique Elements
 */


// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32,i32> = HashMap::with_capacity(nums.len());
        let mut res = 0;
        for x in nums{
            if !map.contains_key(&x) {
                map.insert(x, 0);
                res+=x;
            }else
                 {
                    if *map.get(&x).unwrap() == 0{
                        res-=x;
                        if let Some(y) = map.get_mut(&x){
                            *y = 1;
                        }
                        println!("yo");
                    }
                    
                    
                }
            }
        
        res
    }
}
// @lc code=end

