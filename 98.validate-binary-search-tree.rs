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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn traversal(root: Option<Rc<RefCell<TreeNode>>>, gt: i64, lt: i64) -> bool {
            match root {
                Some(r) => {
                    let b = r.borrow();
                    (b.val as i64) > gt
                        && (b.val as i64) < lt
                        && traversal(b.left.clone(), gt, b.val as i64)
                        && traversal(b.right.clone(), b.val as i64, lt)
                }
                None => true,
            }
        }
        traversal(root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }
}
