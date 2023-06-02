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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root) => {
                let p = p.unwrap();
                let q = q.unwrap();
                if p.borrow().val < root.borrow().val && q.borrow().val < root.borrow().val {
                    return Self::lowest_common_ancestor(
                        root.borrow().left.clone(),
                        Some(p),
                        Some(q),
                    );
                } else if p.borrow().val > root.borrow().val && q.borrow().val > root.borrow().val {
                    return Self::lowest_common_ancestor(
                        root.borrow().right.clone(),
                        Some(p),
                        Some(q),
                    );
                }
                Some(root)
            }
            None => root,
        }
    }
}
