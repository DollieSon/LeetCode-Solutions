use macros::sol_macro;
sol_macro!();

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

use std::rc::Rc;
use std::cell::{Ref, RefCell};
impl Solution {

    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // let root1_leaf = Self::get_leaf(root1);
        // let root2_leaf = Self::get_leaf(root2);
        let mut root1_leaf = Vec::new();
        let mut root2_leaf = Vec::new();
        Self::rec_get_leaf(root1, &mut root1_leaf);
        Self::rec_get_leaf(root2, &mut root2_leaf);
        // println!("{:?}",root1_leaf);
        // println!("{:?}",root2_leaf);
        return root1_leaf == root2_leaf;
    }

    pub fn rec_get_leaf(node:Option<Rc<RefCell<TreeNode>>>, out : &mut Vec<i32>){
        if let Some(n) = node {
            let b = n.borrow();
            if b.left.is_none() && b.right.is_none() {out.push(b.val);}
            else {
                Self::rec_get_leaf(b.left.clone(), out);
                Self::rec_get_leaf(b.right.clone(), out);
            }
        }
    }

    pub fn get_leaf(root:Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        let mut res = Vec::new();
        while queue.is_empty() == false{
            let node = queue.pop().unwrap();
            if let Some(item) = node{
                let thing = item.borrow();
                match (thing.left.clone(),thing.right.clone()) {
                    (None,None) =>{
                        res.push(thing.val);
                    }
                    (left,right)=> {
                        if let Some(x) = left { queue.push(Some(x));}
                        if let Some(x) = right { queue.push(Some(x));}
                    }
                    
                }
            }
        }
        return res;
    }

}