/*
 * @lc app=leetcode id=1859 lang=rust
 *
 * [1859] Sorting the Sentence
 */

// @lc code=start
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut a = s.split(" ").collect::<Vec<&str>>();
        let mut vec: Vec<&str> = vec!["";a.len()];
        for x in a{
            vec[x.get(x.len()-1..x.len()).unwrap().parse::<usize>().unwrap() -1] = x.get(0..x.len()-1).unwrap();
        }
        vec.join(" ")
    }
}
// @lc code=end

