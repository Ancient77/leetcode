/*
 * @lc app=leetcode id=2648 lang=typescript
 *
 * [2648] Generate Fibonacci Sequence
 */

// @lc code=start
function* fibGenerator(): Generator<number, any, number> {
    let x = 0;
    yield x;
    let y = 1;
    yield y;
    while(true){
        [x,y] = [y, x+y];
        yield y;
    }
};

/**
 * const gen = fibGenerator();
 * gen.next().value; // 0
 * gen.next().value; // 1
 */
// @lc code=end

