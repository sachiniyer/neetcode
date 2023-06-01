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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut start = Box::new(ListNode::new(0));
        let mut h = &mut start;

        let mut h1 = l1;
        let mut h2 = l2;

        let mut carry = 0;
        while h1.is_some() && h2.is_some() {
            let mut sum = h1.as_ref().unwrap().val + h2.as_ref().unwrap().val + carry;
            carry = sum / 10;
            sum = sum % 10;

            h.next = Some(Box::new(ListNode::new(sum)));

            let t = h1.as_mut().unwrap().next.take();
            h1 = t;

            let t = h2.as_mut().unwrap().next.take();
            h2 = t;

            h = h.next.as_mut().unwrap();
        }

        if h1.is_some() {
            while h1.is_some() {
                let mut sum = h1.as_ref().unwrap().val + carry;
                carry = sum / 10;
                sum = sum % 10;
                h.next = Some(Box::new(ListNode::new(sum)));

                let t = h1.as_mut().unwrap().next.take();
                h1 = t;

                h = h.next.as_mut().unwrap();
            }
            h.next = h1;
        } else {
            while h2.is_some() {
                let mut sum = h2.as_ref().unwrap().val + carry;
                carry = sum / 10;
                sum = sum % 10;
                h.next = Some(Box::new(ListNode::new(sum)));

                let t = h2.as_mut().unwrap().next.take();
                h2 = t;

                h = h.next.as_mut().unwrap();
            }
            h.next = h2
        }
        if carry != 0 {
            h.next = Some(Box::new(ListNode::new(carry)));
        }
        start.next
    }
}
