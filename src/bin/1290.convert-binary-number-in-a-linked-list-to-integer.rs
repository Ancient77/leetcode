/*
 * @lc app=leetcode id=1290 lang=rust
 *
 * [1290] Convert Binary Number in a Linked List to Integer
 */


// Definition for singly-linked list.
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
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut res: i32 = 0;
        loop {
            match head {
                Some(x) => {
                    res *=2;
                    if(x.val == 1){
                        res+=1;
                    }
                    head = x.next;
                },
                None => return res
            }
            
        }
    }
}
// @lc code=end

