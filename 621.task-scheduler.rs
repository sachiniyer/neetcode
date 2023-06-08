use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut tasks_map: HashMap<char, i32> = HashMap::new();
        for t in tasks {
            *tasks_map.entry(t).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for v in tasks_map.values() {
            heap.push(*v);
        }

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut time = 0;
        while !heap.is_empty() || !queue.is_empty() {
            time += 1;
            match heap.pop() {
                Some(h) => {
                    if h - 1 > 0 {
                        queue.push_front((h - 1, time + n));
                    }
                }
                None => match queue.back() {
                    Some(h) => {
                        time = h.1;
                    }
                    None => (),
                },
            }
            match queue.back() {
                Some(h) => {
                    if h.1 <= time {
                        heap.push(h.0);
                        queue.pop_back();
                    }
                }
                None => (),
            }
        }
        time
    }
}
