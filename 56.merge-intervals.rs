use std::cmp::Reverse;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return intervals;
        }
        let mut intervals = intervals;
        intervals.sort_by_key(|v| (Reverse(v[0]), Reverse(v[1])));
        let mut res = Vec::new();
        res.push(intervals.pop().unwrap());
        while intervals.len() > 0 {
            let next = intervals.pop().unwrap();
            let prev = res.last_mut().unwrap();
            if prev[1] >= next[0] {
                prev[1] = prev[1].max(next[1]);
            } else {
                res.push(next);
            }
        }
        res
    }
}
