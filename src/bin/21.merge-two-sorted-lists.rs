/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

use std::collections::linked_list::Cursor;

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let current1 = list1;
        let current2 = list2;
        let mut result =ListNode::new(0);
        let mut currentResult = &result;
        while current1.is_some() || current2.is_some() {
            if (current2.is_none() || current1.unwrap().val < current2.unwrap().val) {
                match currentResult {
                    Some(x) => x.next = Option::Some(Box::new(ListNode::new(current1.unwrap().val))),
                    None => currentResult.unwrap().next = Option::Some(Box::new(ListNode::new(current1.unwrap().val))),
                };
                currentResult = &currentResult.unwrap().next;
                current1 = current1.unwrap().next;
            } else {
                match currentResult {
                    Some(x) => x.next = Option::Some(Box::new(ListNode::new(current2.unwrap().val))),
                    None => currentResult.unwrap().next = Option::Some(Box::new(ListNode::new(current2.unwrap().val))),
                };
                currentResult = &currentResult.unwrap().next;
                current2 = current2.unwrap().next;
            };
        }

        return result;
    }
}
// @lc code=end
