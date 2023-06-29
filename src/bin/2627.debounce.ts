/*
 * @lc app=leetcode id=2627 lang=typescript
 *
 * [2627] Debounce
 */

// @lc code=start
type F = (...p: any[]) => any

function debounce(fn: F, t: number): F {
    let timeOut;
    return function(...args) {
        clearTimeout(timeOut);
        timeOut = setTimeout(fn,t,...args);
    }
};

/**
 * const log = debounce(console.log, 100);
 * log('Hello'); // cancelled
 * log('Hello'); // cancelled
 * log('Hello'); // Logged at t=100ms
 */
// @lc code=end

