use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Interval(i32, i32);

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ((other.0 - other.1).abs()).cmp(&(self.0 - self.1).abs())
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (v[0], v[1]));
        let mut intervals: VecDeque<Interval> =
            intervals.iter().map(|v| Interval(v[0], v[1])).collect();
        let mut queries: Vec<(i32, usize)> =
            queries.iter().enumerate().map(|(i, e)| (*e, i)).collect();
        queries.sort_by_key(|&(item, _)| item);

        let mut res = vec![-1; queries.len()];
        let mut interval_heap: BinaryHeap<Interval> = BinaryHeap::new();
        for (q, i) in queries.iter() {
            while intervals.len() > 0 && intervals.front().unwrap().0 <= *q {
                interval_heap.push(intervals.pop_front().unwrap());
            }
            while interval_heap.len() > 0 && interval_heap.peek().unwrap().1 < *q {
                interval_heap.pop();
            }

            match interval_heap.peek() {
                Some(v) => {
                    res[*i] = v.1 - v.0 + 1;
                }
                None => res[*i] = -1,
            }
        }
        res
    }
}
