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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut i = 0;
        let mut h = &*head;

        while let Some(v) = h {
            h = &v.next;
            i += 1;
        }

        if i <= 2 {
            return;
        }

        let mut m = head.as_mut();
        let mi = i / 2;
        i = 0;
        while i < mi {
            m = m.unwrap().next.as_mut();
            i += 1;
        }

        let mut m = m.unwrap().next.take();
        let mut p = ListNode::new(0);
        while let Some(mut v) = m.take() {
            m = v.next.take();
            v.next = p.next.take();
            p.next = Some(v);
        }

        let mut t = &mut head.as_mut().unwrap().next;
        while t.is_some() && p.next.is_some() {
            // SAFETY: We know there is a reversed.next because we already checked it
            let mut rev = p.next.take().unwrap();
            p.next = rev.next.take();

            rev.next = t.take();
            *t = Some(rev);
            t = &mut t.as_mut().unwrap().next;
            if let Some(node) = t {
                t = &mut node.next;
            }
        }
    }
}
