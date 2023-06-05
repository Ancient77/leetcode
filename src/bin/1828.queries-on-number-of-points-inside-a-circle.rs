/*
 * @lc app=leetcode id=1828 lang=rust
 *
 * [1828] Queries on Number of Points Inside a Circle
 */

// @lc code=start
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut resVec = Vec::with_capacity(queries.len());
        for i in 0..queries.len(){
            let query = &queries[i];
            resVec.push(0);
            for j in 0..points.len(){
                let point = &points[j];
                if(((point[0] - query[0]).pow(2) + (point[1] - query[1]).pow(2)) as f64) <= query[2].pow(2) as f64{
                    resVec[i]+=1;
                }
            }
        }
        resVec
    }
}
// @lc code=end

