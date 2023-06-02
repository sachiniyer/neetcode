// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match p {
            Some(p) => match q {
                Some(q) => {
                    let right =
                        Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone());
                    let left = Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone());
                    (p.borrow().val == q.borrow().val) && right && left
                }
                None => false,
            },
            None => match q {
                Some(q) => false,
                None => true,
            },
        }
    }
}
