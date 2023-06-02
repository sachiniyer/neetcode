// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut h1 = &mut head;
        for _ in (0..k) {
            if let Some(n) = h1 {
                h1 = &mut n.next;
            } else {
                return head;
            }
        }
        let mut ret = Self::reverse_k_group(h1.take(), k);
        while let Some(h) = head.take() {
            ret = Some(Box::new(ListNode {
                val: h.val,
                next: ret,
            }));
            head = h.next;
        }
        ret
    }
}
