use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (Reverse(v[0]), Reverse(v[1])));

        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        heap.push(Reverse(intervals.pop().unwrap()[1]));
        let mut res = 1;
        while intervals.len() > 0 {
            let next = intervals.pop().unwrap();
            if heap.len() == 0 {
                heap.push(Reverse(next[1]));
            } else if next[0] >= heap.peek().unwrap().0 {
                heap.pop();
                heap.push(Reverse(next[1]))
            } else {
                heap.push(Reverse(next[1]))
            }
            res = res.max(heap.len())
        }
        res as i32
    }
}
