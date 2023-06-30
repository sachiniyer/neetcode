use std::cmp::Reverse;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (Reverse(v[0]), Reverse(v[1])));
        let mut end = intervals.pop().unwrap()[1];
        let mut res = 0;
        while intervals.len() > 0 {
            let next = intervals.pop().unwrap();
            if next[0] < end {
                res += 1;
                end = end.min(next[1]);
            } else {
                end = next[1];
            }
        }
        res
    }
}
