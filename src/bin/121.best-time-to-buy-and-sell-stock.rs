/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let pricelen = prices.len() -1;
        let mut LowestIndex = vec![pricelen];
        let mut max = 0;
        for n in 0..=pricelen {
            if(prices.get(pricelen-n).unwrap() > prices.get(*LowestIndex.get(LowestIndex.len()-1).unwrap() as usize).unwrap()){
                LowestIndex.push(pricelen-n);
            }
        }
        LowestIndex.reverse();
        for i in 0..=pricelen{
            if LowestIndex[0] == i{
                LowestIndex.remove(0);
            }
            if(LowestIndex.len() == 0){
                continue;
            }
            if((prices[LowestIndex[0]] - prices[i]) > max){
                max = prices[LowestIndex[0]] - prices[i];
            }
        }
        max
    }
}
// @lc code=end

