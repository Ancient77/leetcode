/*
 * @lc app=leetcode id=551 lang=rust
 *
 * [551] Student Attendance Record I
 */

// @lc code=start
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut ACounter = 0;
        let mut LCounter = 0;
        for c in s.chars(){
            match c {
              'P' =>  {
                LCounter = 0;
              },
              'A' => {
                LCounter = 0;
                ACounter+=1;
              },
              'L' => {
                LCounter+=1;
              },
              _ => {}
            }
            if(ACounter == 2){return false;}
            if(LCounter == 3){return false;}
        }
        return true;
    }
}
// @lc code=end

