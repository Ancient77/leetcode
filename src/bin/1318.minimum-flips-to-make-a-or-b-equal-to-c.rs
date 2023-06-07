/*
 * @lc app=leetcode id=1318 lang=rust
 *
 * [1318] Minimum Flips to Make a OR b Equal to c
 */

// @lc code=start
impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut count = 0;
        
        while (a!=0 || b != 0 || c != 0) {
            if(c % 2 == 0){
                if(a % 2 == 1){count+=1;}
                if(b % 2 == 1){count+=1;}
            }else {
                if(a % 2 == 0 && b % 2 == 0){
                    count+=1;
                }
            }
            a = a / 2;
            c = c / 2;
            b = b / 2;
        }
        
        count

    }
}
// @lc code=end

