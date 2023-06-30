use std::cmp::Reverse;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() == 0 {
            return true;
        }
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (Reverse(v[0]), Reverse(v[1])));
        let mut prev = intervals.pop().unwrap();
        while intervals.len() > 0 {
            let next = intervals.pop().unwrap();
            if next[0] < prev[1] {
                return false;
            }
            prev = next;
        }
        true
    }
}
