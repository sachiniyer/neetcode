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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut start = Box::new(ListNode::new(0));
        let mut h = &mut start;

        let mut h1 = list1;
        let mut h2 = list2;

        while h1.is_some() && h2.is_some() {
            if h1.as_ref().unwrap().val < h2.as_ref().unwrap().val {
                let t = h1.as_mut().unwrap().next.take();
                h.next = h1;
                h1 = t;
            } else {
                let t = h2.as_mut().unwrap().next.take();
                h.next = h2;
                h2 = t;
            }
            h = h.next.as_mut().unwrap();
        }

        if h1.is_none() {
            h.next = h2;
        } else {
            h.next = h1;
        }
        return start.next;
    }
}
