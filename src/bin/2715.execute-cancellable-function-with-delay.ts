/*
 * @lc app=leetcode id=2715 lang=typescript
 *
 * [2715] Execute Cancellable Function With Delay
 */

// @lc code=start
function cancellable(fn: Function, args: any[], t: number): Function {
    let timeout = setTimeout(fn, t,...args);
    return () => clearTimeout(timeout); 

};

/**
 *  const result = []
 *
 *  const fn = (x) => x * 5
 *  const args = [2], t = 20, cancelT = 50
 *
 *  const log = (...argsArr) => {
 *      result.push(fn(...argsArr))
 *  }
 *       
 *  const cancel = cancellable(fn, args, t);
 *           
 *  setTimeout(() => {
 *     cancel()
 *     console.log(result) // [{"time":20,"returned":10}]
 *  }, cancelT)
 */
// @lc code=end

