use std::arch::x86_64::_MM_FROUND_NO_EXC;

use macros::sol_macro;
sol_macro!();

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

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut x = 0;
        return Self::del_rec(head, 0, &mut x);
    }
    pub fn del_rec(head: Option<Box<ListNode>>,ind:i32,mid:&mut i32) ->Option<Box<ListNode>> {
        match head {
            None => {
                *mid = ind/2;
                None
            }
            Some(mut curr) => {
                let next = Self::del_rec(curr.next, ind+1, mid);
                if ind == *mid {
                    next
                }else {
                    curr.next = next;
                    Some(curr)
                }

            }
        }
    }

}