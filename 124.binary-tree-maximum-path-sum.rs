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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, m: &mut i32) -> i32 {
            match node {
                Some(n) => {
                    let right = traversal(n.borrow().right.clone(), m).max(0);
                    let left = traversal(n.borrow().left.clone(), m).max(0);
                    *m = (*m).max(right + left + n.borrow().val);
                    (right + n.borrow().val).max(left + n.borrow().val)
                }
                None => 0,
            }
        }
        let mut m: i32 = i32::MIN;
        traversal(root, &mut m);
        m
    }
}
