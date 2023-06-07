/*
 * @lc app=leetcode id=2666 lang=typescript
 *
 * [2666] Allow One Function Call
 */

// @lc code=start
function once<T extends (...args: any[]) => any>(fn: T): 
 ((...args: Parameters<T>) => ReturnType<T> | undefined) {
  let executed = false;
  return function (...args) {
    if(executed){return undefined;}
    executed = true;
    return fn(...args)
  };
}

/**
 * let fn = (a,b,c) => (a + b + c)
 * let onceFn = once(fn)
 *
 * onceFn(1,2,3); // 6
 * onceFn(2,3,6); // returns undefined without calling fn
 */
// @lc code=end

