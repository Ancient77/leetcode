/*
 * @lc app=leetcode id=2649 lang=typescript
 *
 * [2649] Nested Array Generator
 */

// @lc code=start
type MultidimensionalArray = (MultidimensionalArray | number)[]

function* inorderTraversal(arr: MultidimensionalArray): Generator<number, void, unknown> {
    for (let x of arr){
        if(typeof x == "number"){
            yield x;
        }else{
            yield* inorderTraversal(x);
        }
    }
};

/**
 * const gen = inorderTraversal([1, [2, 3]]);
 * gen.next().value; // 1
 * gen.next().value; // 2
 * gen.next().value; // 3
 */
// @lc code=end

