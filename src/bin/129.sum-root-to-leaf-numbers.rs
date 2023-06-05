/*
 * @lc app=leetcode id=129 lang=rust
 *
 * [129] Sum Root to Leaf Numbers
 */


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
            right: None,
        }
    }
}
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

use std::borrow::Borrow;
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }
use std::cell::{RefCell, Ref};
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return next(root.unwrap().borrow(),vec![root.unwrap().borrow().val]);
    }
    
}
fn next(node: Ref<TreeNode>, sum: Vec<i32>) -> i32 {
    match (node.left.borrow(), node.right.borrow()) {
        (None, None) => {
            let mut s = String::new();

            for i in sum {
                s.push_str(&i.to_string());
            }
            return s.parse::<i32>().unwrap();
        }
        (Some(x), None) | (None, Some(x)) => {
            sum.push(node.val);
            return next(x.borrow(), sum.clone());
        }
        (Some(x), Some(y)) => {
            sum.push(node.val);
            return next(x.borrow(), sum.clone()) + next(y.borrow(), sum.clone());
        }
    }
}
// @lc code=end
