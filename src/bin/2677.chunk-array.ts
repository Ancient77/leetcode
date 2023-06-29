/*
 * @lc app=leetcode id=2677 lang=typescript
 *
 * [2677] Chunk Array
 */

// @lc code=start
function chunk(arr: any[], size: number): any[][] {
    let res:any[][] = [];
    
    for (let index = 0; index < arr.length; index+=size) {
        res.push(arr.slice(index,index+size))
    }
    return res;
};

// @lc code=end

