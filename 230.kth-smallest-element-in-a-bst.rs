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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn kth_smallest(mut root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut stack = VecDeque::new();

        loop {
            while let Some(n) = root {
                stack.push_front(n.clone());
                root = n.borrow().left.clone();
            }

            if let Some(n) = stack.pop_front() {
                k -= 1;

                if k == 0 {
                    return n.borrow().val;
                }

                root = n.borrow().right.clone()
            }
        }
    }
}
