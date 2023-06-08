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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self{}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut q = VecDeque::<Option<Rc<RefCell<TreeNode>>>>::new();
        q.push_back(root);

        let mut v = vec![];

        while let Some(node_opt) = q.pop_front() {
            v.push(match node_opt {
                None => "#".to_string(),
                Some(node_rc) => {
                    let mut node_ref = node_rc.borrow_mut();
                    q.push_back(node_ref.left.take());
                    q.push_back(node_ref.right.take());
                    node_ref.val.to_string()
                }
            });
        }

        v.join(",")
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = data.split(',').map(|x| match x {
            "#" => None,
             _ => {
                 Some(Rc::new(RefCell::new(TreeNode::new(x.parse::<i32>().ok().unwrap()))))
             }
        }).collect();
        let mut slow = 0;
        let mut fast = 1;
        let n = nodes.len();

        while fast < n {
            let m = fast;
            while slow < m {
                if let Some(nrc) = &nodes[slow] {
                    nrc.borrow_mut().left = nodes[fast].clone();
                    nrc.borrow_mut().right = nodes[fast + 1].clone();
                    fast += 2;
                }
                slow += 1;
            }
        }
        nodes[0].clone()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
