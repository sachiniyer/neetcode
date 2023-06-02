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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        fn traversal(
            root: Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            res: &mut Vec<i32>,
            max_depth: &mut i32,
        ) {
            match root {
                Some(r) => {
                    if depth > *max_depth {
                        *max_depth = depth;
                        res.push(r.borrow().val);
                    }
                    traversal(r.borrow().right.clone(), depth + 1, res, max_depth);
                    traversal(r.borrow().left.clone(), depth + 1, res, max_depth);
                }
                None => return,
            }
        }

        let mut max_depth = -1;
        traversal(root, 0, &mut res, &mut max_depth);
        res
    }
}
