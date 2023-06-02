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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traversal(root: Option<Rc<RefCell<TreeNode>>>, min_val: i32) -> i32 {
            match root {
                Some(r) => {
                    let r = r.borrow();
                    let mut new_min_val = min_val;
                    let mut add = 0;
                    if r.val >= min_val {
                        new_min_val = r.val;
                        add = 1;
                    }
                    add + traversal(r.left.clone(), new_min_val)
                        + traversal(r.right.clone(), new_min_val)
                }
                None => 0,
            }
        }
        traversal(root, -10001)
    }
}
