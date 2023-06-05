/*
 * @lc app=leetcode id=1402 lang=rust
 *
 * [1402] Reducing Dishes
 */

// @lc code=start
impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        //sort positive by value 

        //for negative keep positiv sum and then check if sum > 0 if you add the next biggest negative
        //if yes sum - (-int) 
        satisfaction.sort();
        let resIter = satisfaction.iter().filter(|x| **x > 0);
        let mut sum:i32 = resIter.clone().sum();
        
        let mut negIter = satisfaction.iter().filter(|x| **x<=0).rev();
        let mut negRes = vec![];
        for neg in negIter {
            sum = sum + neg;
            if(sum > 0 ){
                negRes.push(neg);
            }else{
                break
            }
        }
        let mut counter = 1;
        let mut res = 0;
        
        for x in negRes.iter().rev(){
            res = res + counter*(*x);
            counter+=1;
        }
        for x in resIter{
            res = res + counter*x;
            counter+=1;
        }
        res
        
    }
}
// @lc code=end

