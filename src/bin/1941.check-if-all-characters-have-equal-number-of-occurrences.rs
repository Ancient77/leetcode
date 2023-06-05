/*
 * @lc app=leetcode id=1941 lang=rust
 *
 * [1941] Check if All Characters Have Equal Number of Occurrences
 */

// @lc code=start

use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map:HashMap<char,i32> = HashMap::with_capacity(s.len());
        s.chars().for_each(|x| {
            *map.entry(x).or_insert(0)+=1;
        });
        let (_,x) = map.iter().next().unwrap();
        !map.iter().any(|(k,v)| x != v)
    }
}
// @lc code=end

