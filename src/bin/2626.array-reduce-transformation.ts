/*
 * @lc app=leetcode id=2626 lang=typescript
 *
 * [2626] Array Reduce Transformation
 */

// @lc code=start
type Fn = (accum: number, curr: number) => number

function reduce(nums: number[], fn: Fn, init: number): number {
    nums.forEach((x) => {
        init = fn(init,x);
    })
    return init;
};
// @lc code=end

