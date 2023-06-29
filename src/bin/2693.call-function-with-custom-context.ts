/*
 * @lc app=leetcode id=2693 lang=typescript
 *
 * [2693] Call Function with Custom Context
 */

// @lc code=start
//@ts-ignore
declare global { 
    interface Function {
      callPolyfill(context: Record<any, any>, ...args: any[]): any;
	}
}
//@ts-ignore
Function.prototype.callPolyfill = function(context: Record<any, any>, ...args: any[]): any {
  let fn = this;
  Object.defineProperty(context, 'func', {
    value: fn,
    enumerable: false
  });
  return context.func(...args);
}

/**
 * function increment() { this.count++; return this.count; }
 * increment.callPolyfill({count: 1}); // 2
 */
// @lc code=end

