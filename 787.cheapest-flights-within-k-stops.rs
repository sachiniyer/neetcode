// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// use std::collections::HashMap;
// use std::collections::VecDeque;

// #[derive(Debug, Eq, Clone, Copy)]
// struct Node {
//     val: i32,
//     cost: i32,
//     stops: i32,
// }

// impl Node {
//     fn new(val: i32, cost: i32, stops: i32) -> Self {
//         Node { val, cost, stops }
//     }
// }

// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }
// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(other.cost.cmp(&self.cost))
//     }
// }

// impl PartialEq for Node {
//     fn eq(&self, other: &Self) -> bool {
//         self.cost == other.cost
//     }
// }

// #[derive(Debug, Eq, Clone, Copy)]
// struct Edge {
//     val: i32,
//     cost: i32,
// }

// impl Edge {
//     fn new(val: i32, cost: i32) -> Self {
//         Edge { val, cost }
//     }
// }

// impl Ord for Edge {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }
// impl PartialOrd for Edge {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(other.cost.cmp(&self.cost))
//     }
// }

// impl PartialEq for Edge {
//     fn eq(&self, other: &Self) -> bool {
//         self.cost == other.cost
//     }
// }

// impl Solution {
//     pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
//         let mut flight_map: HashMap<i32, BinaryHeap<Edge>> = HashMap::new();
//         for f in flights {
//             flight_map
//                 .entry(f[0])
//                 .or_insert(BinaryHeap::new())
//                 .push(Edge::new(f[1], f[2]));
//         }
//         let mut heap: BinaryHeap<Node> = BinaryHeap::new();
//         heap.push(Node::new(src, 0, 0));
//         let mut visited: HashMap<i32, i32> = HashMap::new();
//         let mut cost = i32::MAX;
//         while let Some(n) = heap.pop() {
//             // println!("===============================");
//             // println!("{:?}", heap);
//             // println!("{:?}", n);
//             if n.val == dst {
//                 cost = cost.min(n.cost);
//             } else if n.stops <= k {
//                 if let Some(flights) = flight_map.get(&n.val) {
//                     // println!("{:?}", flights);
//                     let mut flights: BinaryHeap<Edge> = flights.clone();
//                     while flights.len() > 0 {
//                         let f = flights.pop().unwrap();
//                         if let Some(v) = visited.get_mut(&f.val) {
//                             if v > n.stops + 1 {
//                                 v.1 = v.1.min(n.stops + 1);
//                                 heap.push(Node::new(f.val, n.cost + f.cost, n.stops + 1));
//                             }
//                         } else {
//                             visited.insert(f.val, (n.cost + f.cost, n.stops + 1));
//                             heap.push(Node::new(f.val, n.cost + f.cost, n.stops + 1));
//                         }
//                     }
//                 }
//             }
//             // println!("{:?}", heap);
//         }
//         if cost == i32::MAX {
//             -1
//         } else {
//             cost
//         }
//     }
// }
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Clone)]
struct Edge {
    to: usize,
    weight: i32,
}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (src, dst, n, k) = (src as usize, dst as usize, n as usize, 1 + k as usize);

        let graph = flights.iter().fold(vec![vec![]; n], |mut g, flight| {
            g[flight[0] as usize].push(Edge {
                to: flight[1] as usize,
                weight: flight[2],
            });
            g
        });

        // on each node, a size k + 1 vector is used to record the
        // shortest distance from src
        // dist[i][j]: is the shortest distance from src of the i-th (0-based) node
        //             with j stops
        let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX; k]; n];

        // This is a standard Dijkstra algorithm
        // Use Reverse to make it a min heap
        let mut pq = BinaryHeap::new();

        for j in 0..k {
            dist[src][j] = 0;
        }
        pq.push((Reverse(0), src, 0));
        while let Some((Reverse(d), u, cnt)) = pq.pop() {
            if u == dst {
                return d;
            }
            if cnt == k {
                continue;
            }

            for p in &graph[u] {
                if dist[p.to][cnt] > p.weight + d {
                    dist[p.to][cnt] = p.weight + d;
                    pq.push((Reverse(p.weight + d), p.to, cnt + 1));
                }
            }
        }
        -1
    }
}
