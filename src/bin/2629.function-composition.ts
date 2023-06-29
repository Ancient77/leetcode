/*
 * @lc app=leetcode id=2629 lang=typescript
 *
 * [2629] Function Composition
 */

// @lc code=start
type F = (x: number) => number;

function compose(functions: F[]): F {
    let revFunctions = functions.reverse();
	return function(x) {
        revFunctions.forEach((f) => {
            x = f(x);
        })
        return x;
    }
};

/**
 * const fn = compose([x => x + 1, x => 2 * x])
 * fn(4) // 9
 */
// @lc code=end

