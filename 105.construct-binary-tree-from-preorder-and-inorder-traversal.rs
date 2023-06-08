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

// preorder tells you what you will
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(
            preorder: &[i32],
            inorder: &[i32],
            map: &HashMap<i32, usize>,
            pre_start: usize,
            pre_end: usize,
            in_start: usize,
            in_end: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if pre_start >= pre_end || in_start >= in_end {
                return None;
            }
            let root_val = preorder[pre_start];
            let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
            let root_idx = map[&root_val];
            let left_size = root_idx - in_start;
            root.borrow_mut().left = build(
                preorder,
                inorder,
                map,
                pre_start + 1,
                pre_start + 1 + left_size,
                in_start,
                root_idx,
            );
            root.borrow_mut().right = build(
                preorder,
                inorder,
                map,
                pre_start + 1 + left_size,
                pre_end,
                root_idx + 1,
                in_end,
            );
            Some(root)
        }
        let mut map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            map.insert(val, i);
        }
        build(
            &preorder,
            &inorder,
            &map,
            0,
            preorder.len(),
            0,
            inorder.len(),
        )
    }
}
