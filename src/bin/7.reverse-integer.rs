/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let isNeg = x < 0;
        let mut newX: i32 = 0;
        x = x.abs();
        loop {
            if(x == 0){break;}
            match newX.checked_mul(10) {
                Some(x) => newX = x,
                None => return 0
            }
            newX += x % 10;
            x = (x - x % 10) / 10;
        }
        return if(isNeg){newX*-1}else{newX}
    }
}
// @lc code=end
