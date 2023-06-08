use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Point(i32, i32);

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut d1 = ((self.0.abs().pow(2) as f64) + (self.1.abs().pow(2) as f64)).sqrt();
        let mut d2 = ((other.0.abs().pow(2) as f64) + (other.1.abs().pow(2) as f64)).sqrt();
        d2.partial_cmp(&d1).unwrap()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<Reverse<Point>> = BinaryHeap::new();
        for i in points {
            heap.push(Reverse(Point(i[0], i[1])));
        }
        while heap.len() > k as usize {
            heap.pop();
        }
        let mut res: Vec<Vec<i32>> = Vec::new();
        while !heap.is_empty() {
            if let Some(Reverse(i)) = heap.pop() {
                res.push(vec![i.0, i.1]);
            }
        }
        res
    }
}
