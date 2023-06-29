/*
 * @lc app=leetcode id=2631 lang=typescript
 *
 * [2631] Group By
 */

// @lc code=start
//@ts-ignore
declare global {
    interface Array<T> {
        groupBy(fn: (item: T) => string): Record<string, T[]>
    }
}
//@ts-ignore
Array.prototype.groupBy = function(fn) {
    let map = new Map();
    this.forEach(x => {
        if(map.has(fn(x))){
            map.get(fn(x)).push(x);
        }else{
            map.set(fn(x), [x]);
        }
    });
    return Object.fromEntries(map);
}
/**
 * [1,2,3].groupBy(String) // {"1":[1],"2":[2],"3":[3]}
 */
// @lc code=end

