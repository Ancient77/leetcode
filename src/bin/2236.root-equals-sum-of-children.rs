/*
 * @lc app=leetcode id=2236 lang=rust
 *
 * [2236] Root Equals Sum of Children
 */


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let rootVal = root.as_ref().unwrap().clone().borrow().val;
    let leftVal = root.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().val.clone();
    let rightVal = root.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().val.clone();
    rootVal == leftVal + rightVal
    }
}
// @lc code=end

