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
    pub fn is_subtree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match p {
            Some(p) => match q {
                Some(q) => {
                    let mut e = false;
                    let r2 = Self::is_subtree(p.borrow().right.clone(), Some(q.clone()));
                    let l2 = Self::is_subtree(p.borrow().left.clone(), Some(q.clone()));

                    if p.borrow().val == q.borrow().val {
                        e = Self::is_same_tree(Some(p.clone()), Some(q.clone()));
                    }
                    e || r2 || l2
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
