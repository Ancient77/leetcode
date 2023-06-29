/*
 * @lc app=leetcode id=2695 lang=typescript
 *
 * [2695] Array Wrapper
 */

// @lc code=start
class ArrayWrapper {
	nums:number[];
    constructor(nums: number[]) {
        this.nums=nums;
    }

	valueOf() {
        return this.nums.reduce((x,y) => x+y,0);
    }

	toString() {
        return `[${this.nums.toString()}]`;
    }
};

/**
 * const obj1 = new ArrayWrapper([1,2]);
 * const obj2 = new ArrayWrapper([3,4]);
 * obj1 + obj2; // 10
 * String(obj1); // "[1,2]"
 * String(obj2); // "[3,4]"
 */
// @lc code=end

