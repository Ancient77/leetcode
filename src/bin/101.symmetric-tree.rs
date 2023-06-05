/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq,Clone)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn compare(left:Option<Rc<RefCell<TreeNode>>>,right:Option<Rc<RefCell<TreeNode>>>) -> bool{
            match (left,right){
                (None,None) => true,
                (None,Some(_)) | (Some(_), None) => false,
                (Some(left),Some(right)) => {
                    if(left.borrow().val != right.borrow().val){return false};
                    return compare(left.borrow().left.clone(), right.borrow().right.clone()) && compare(left.borrow().right.clone(), right.borrow().left.clone());
                }
            }
        }
        match root{
            None => true,
            Some(x) => compare(x.borrow().left.clone(), x.borrow().right.clone())
        }
    }
}

// @lc code=end

