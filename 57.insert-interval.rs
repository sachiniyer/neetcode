impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::{max, min};
        let mut less = vec![];
        let mut more = vec![];
        let mut start = new_interval[0];
        let mut end = new_interval[1];

        for curr in intervals {
            if curr[1] < new_interval[0] {
                less.push(curr);
            } else if curr[0] > new_interval[1] {
                more.push(curr);
            } else {
                start = min(curr[0], start);
                end = max(curr[1], end);
            }
        }
        less.push(vec![start, end]);
        less.append(&mut more);
        less
    }
}
