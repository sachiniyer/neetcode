use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Rev(i32);

impl Ord for Rev {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for Rev {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Rev>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new()
        }
    }
    
    fn add_num(&mut self, num: i32) {
        self.left.push(num);
        self.right.push(Rev(*self.left.peek().unwrap()));
        self.left.pop();
        while self.right.len() > self.left.len() {
            self.left.push(self.right.pop().unwrap().0);
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.left.len() > self.right.len() {
            return *self.left.peek().unwrap() as f64;
        }
        (self.right.peek().unwrap().0 as f64 + *self.left.peek().unwrap() as f64) / 2.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
