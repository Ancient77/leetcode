/*
 * @lc app=leetcode id=278 lang=rust
 *
 * [278] First Bad Version
 */

// @lc code=start
// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lowestBad = n;
        let mut highestGood = 0;
        
        loop{
            if(lowestBad - highestGood == 1){return lowestBad;}
            let temp = (lowestBad - highestGood) / 2 + highestGood;
            if(self.isBadVersion(temp)){
                lowestBad = temp;
            }else{
                highestGood = temp;
            }
        }
        
    }
}
// @lc code=end

