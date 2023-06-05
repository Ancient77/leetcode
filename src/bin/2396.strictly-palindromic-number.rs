/*
 * @lc app=leetcode id=2396 lang=rust
 *
 * [2396] Strictly Palindromic Number
 */

// @lc code=start
impl Solution {
    pub fn is_strictly_palindromic(n: i32) -> bool {
        for x in 2..=n-2{
            let vec1:Vec<i32> = base_10_to_base_x(n, x);
            print!("{:?}",vec1);
            if(vec1.len() % 2 == 1){
                return false;
            }
            for n in 0..vec1.len(){
                if(vec1[n] != vec1[vec1.len() - n - 1]){
                    return false;
                }
            }
        }
        true
    }
}
fn base_10_to_base_x(mut n: i32,x:i32) -> Vec<i32> {
    let mut result = vec![];
    while n > 0 {
        result.push( (n % x));
        n /= x;
    }
    result.reverse();
    result
}
// @lc code=end

