/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut sum = 0;
        if(flowerbed.len() == 1){
            match flowerbed[0]{
                0 => return (n-1) <= 0,
                1 => return n == 0,
                _ => ()
            }
        }
        for i in 0..flowerbed.len() {
            if (flowerbed[i] == 1) {
                continue;
            }
            
            if (i == 0) {
                if (flowerbed[i + 1] == 0) {
                    flowerbed[i] = 1;
                    sum += 1;
                    
                }
                continue;
            }
            if (i == flowerbed.len() - 1) {
                if (flowerbed[i - 1] == 0) {
                    flowerbed[i] = 1;
                    sum += 1;
                    
                }
                continue;
            }
            if (flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0) {
                flowerbed[i] = 1;
                sum += 1;
                continue;
            }
        }
        return sum >= n;
    }
}
// @lc code=end
