/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */


//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
// @lc code=start
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_lists(&l1,&l2,0)
    }
    pub fn add_lists(l1: &Option<Box<ListNode>>,l2: &Option<Box<ListNode>>,c:i32) -> Option<Box<ListNode>> {
        match (l1,l2,c){
            (None,None,0) => None,
            (None,None,c) => Some(Box::new(ListNode::new(c))),
            (Some(l),None,c) | (None,Some(l),c)=>{
                let sum = l.val + c;
                Some(Box::new(ListNode{val:sum%10, next: Self::add_lists(&l.next,&None,sum /10)}))
            },
            (Some(l1),Some(l2),c) => {
                let sum = l1.val + l2.val + c;
                Some(Box::new(ListNode{val:sum%10,next:Self::add_lists(&l1.next, &l2.next, sum/10)}))
            }
        }
    }
}
// @lc code=end

