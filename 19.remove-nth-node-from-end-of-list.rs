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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut d = ListNode::new(0);
        d.next = head;
        let mut d = Box::new(d);
        let mut p2 = d.clone();
        let mut p1 = d.as_mut();
        for _ in (0..n) {
            p2 = p2.next.unwrap()
        }
        while p2.next.is_some() {
            p1 = p1.next.as_mut().unwrap();
            p2 = p2.next.unwrap();
        }
        let mut nl = p1.next.as_mut().unwrap();
        p1.next = nl.next.clone();
        d.next
    }
}
