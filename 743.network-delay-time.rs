use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for edge in times {
            let start = edge[0];
            let end = edge[1];
            let cost = edge[2];
            graph.entry(start).or_insert(vec![]).push((end, cost));
        }
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), k));
        let mut visited: HashMap<i32, i32> = HashMap::new();
        while let Some((Reverse(outer_cost), node)) = heap.pop() {
            if visited.contains_key(&node) {
                continue;
            }
            visited.insert(node, outer_cost);
            if let Some(neighbors) = graph.get(&node) {
                for (neighbor, cost) in neighbors {
                    heap.push((
                        Reverse(
                            *visited
                                .get(neighbor)
                                .unwrap_or(&i32::MAX)
                                .min(&(cost + outer_cost)),
                        ),
                        *neighbor,
                    ));
                }
            }
        }
        if visited.len() == n as usize {
            visited.values().max().unwrap().clone()
        } else {
            -1
        }
    }
}
