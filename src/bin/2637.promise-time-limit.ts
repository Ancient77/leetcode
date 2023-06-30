/*
 * @lc app=leetcode id=2637 lang=typescript
 *
 * [2637] Promise Time Limit
 */

// @lc code=start
type Fn = (...params: any[]) => Promise<any>;

function timeLimit(fn: Fn, t: number): Fn {
    return async function (...args) {
        return Promise.race([
            fn(...args),
            new Promise((_,reject) => setTimeout(reject, t,"Time Limit Exceeded"))
        ])
    }
};

/**
 * const limited = timeLimit((t) => new Promise(res => setTimeout(res, t)), 100);
 * limited(150).catch(console.log) // "Time Limit Exceeded" at t=100ms
 */
// @lc code=end

