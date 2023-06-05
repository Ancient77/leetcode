/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */
struct Solution{

}


// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if(nums.len() == 1){
            return vec![vec![nums[0]]];
        }else{
            for x in 0..nums.len(){
                
                let mut temp2 = nums.clone();
                temp2.remove(x);
                
                for perm in Self::permute(temp2.clone()){
                    let mut temp3 = perm;
                    temp3.append(&mut vec![nums[x]]);
                    res.push(temp3);
                }
            }
        }
        res
    }
}
// @lc code=end

