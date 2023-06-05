/*
 * @lc app=leetcode id=1945 lang=rust
 *
 * [1945] Sum of Digits of String After Convert
 */

// @lc code=start
impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut temp = 0;
        for x in s.chars(){
            temp+= sumOfDigits((x as i32 -96))
        }
        print!("{}", temp);
        let mut sum = temp;
        
        for i in 1..k {
            sum = sumOfDigits(sum)
        }
        sum as i32
    }

}
fn sumOfDigits(mut x:i32) -> i32{
    let mut temp = 0;
    while x != 0 {
        temp+= x % 10;
        x = x / 10;
    }
    temp
}
// @lc code=end

