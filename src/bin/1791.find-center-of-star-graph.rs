/*
 * @lc app=leetcode id=1791 lang=rust
 *
 * [1791] Find Center of Star Graph
 */

// @lc code=start
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if(edges.iter().all(|x| x.contains(&edges[0][0]))){
            return edges[0][0];
        }
        if(edges.iter().all(|x| x.contains(&edges[0][1]))){
            return edges[0][1];
        }
        return 0;
    }
}
// @lc code=end

