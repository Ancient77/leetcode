/*
 * @lc app=leetcode id=1773 lang=rust
 *
 * [1773] Count Items Matching a Rule
 */

// @lc code=start
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut c = 0;
        for item in items{
            match unsafe { rule_key.get_unchecked(0..=0) } {
                "t" => if(rule_value == item[0]){c+=1;},
                "c" => if(rule_value == item[1]){c+=1;},
                "n" => if(rule_value == item[2]){c+=1;},
                _ => ()
            }
        }
        c
    }
}
// @lc code=end

