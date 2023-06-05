/*
 * @lc app=leetcode id=71 lang=rust
 *
 * [71] Simplify Path
 */



// @lc code=start
impl Solution {
    pub fn simplify_path(mut path: String) -> String {
        let mut pathIt = path.splitn(10000, "/");
        let mut resVec = vec![];
        resVec.push(pathIt.next().unwrap());
        while let Some(x) = pathIt.next() {
            if(x.chars().count() == 0){
                continue;
            }
            if(x == "."){
                continue;
            }
            if(x == ".."){
                resVec.pop();
                continue;
            }
            resVec.push(x);
        }
        if(resVec.len() == 0){resVec.push("")}
        if(resVec[0] != ""){
            resVec.insert(0, "");
        }
        path = resVec.join("/");
        if(path.len() == 0){
            return String::from("/");
        }
        
        



        path
    }
}
// @lc code=end

