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

struct KthLargest {
    heap: BinaryHeap<Rev>,
    k: i32
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap: BinaryHeap<Rev> = BinaryHeap::new();
        for n in nums { heap.push(Rev(n)) }

        while heap.len() > k as usize {
            heap.pop();
        }
        Self {
            heap,
            k
        }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Rev(val));
        while self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        if let Some(Rev(v)) = self.heap.peek() {
            return *v;;
        }
        return 0;
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
