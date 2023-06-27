use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
struct Edge {
    start: (i32, i32),
    end: (i32, i32),
    cost: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edge({:?} -> {:?}, cost: {})\n",
            self.start, self.end, self.cost
        )
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let combined_string = format!("{:?}{:?}{:?}", self.start, self.end, self.cost);
        combined_string.hash(state);
    }
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.start == other.start && self.end == other.end
    }
}

impl Edge {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let cost = (start.0 - end.0).abs() + (start.1 - end.1).abs();
        Self { start, end, cost }
    }
}

#[derive(Eq, Clone, Copy, Ord, PartialOrd)]
struct EdgeWrapper {
    edge: Edge,
}

impl EdgeWrapper {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self {
            edge: Edge::new(start, end),
        }
    }
}

impl PartialEq for EdgeWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.edge.cost == other.edge.cost
    }
}

impl Debug for EdgeWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "EdgeWrapper({:?})", self.edge)
    }
}

#[derive(Debug)]
struct EdgeHeap {
    pub heap: BinaryHeap<EdgeWrapper>,
}

impl EdgeHeap {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn add(&mut self, start: (i32, i32), end: (i32, i32)) {
        self.heap.push(EdgeWrapper::new(start, end));
    }

    fn pop(&mut self) -> Option<Edge> {
        match self.heap.pop() {
            Some(wrapper) => Some(wrapper.edge),
            None => None,
        }
    }
}

struct Components {
    map: HashMap<i32, HashSet<Edge>>,
    available_id: i32,
    edges: usize,
}

impl Debug for Components {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Components: \n")?;
        for (id, set) in self.map.iter() {
            write!(f, "id: {}, set: {:?}\n", id, set)?;
        }
        Ok(())
    }
}

impl Components {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            available_id: 0,
            edges: 0,
        }
    }

    fn add(&mut self, edge: Edge, id: i32) {
        match self.map.get_mut(&id) {
            Some(set) => {
                set.insert(edge);
                self.edges += 1;
            }
            None => unreachable!(),
        }
    }

    fn merge(&mut self, set1: i32, set2: i32, edge: Edge) {
        let add_set = self.map.get(&set2).unwrap_or(&HashSet::new()).clone();
        (*self.map.entry(set1).or_insert(HashSet::new())).extend(add_set.iter().cloned());
        self.map.remove(&set2);
        self.add(edge, set1);
    }

    fn insert(&mut self, edge: Edge) -> i32 {
        let mut id = self.available_id;
        self.map.insert(id, vec![edge].into_iter().collect());
        self.available_id += 1;
        self.edges += 1;
        id
    }

    fn find(&mut self, point: (i32, i32)) -> Option<i32> {
        for (id, set) in self.map.iter() {
            for e in set.iter() {
                if e.start == point || e.end == point {
                    return Some(*id);
                }
            }
        }
        None
    }

    fn add_edge(&mut self, edge: Edge) {
        let start_id = self.find(edge.start);
        let end_id = self.find(edge.end);
        match (start_id, end_id) {
            (Some(start_id), Some(end_id)) => {
                if start_id != end_id {
                    self.merge(start_id, end_id, edge);
                }
            }
            (Some(start_id), None) => {
                self.add(edge, start_id);
            }
            (None, Some(end_id)) => {
                self.add(edge, end_id);
            }
            (None, None) => {
                self.insert(edge);
            }
        }
    }

    fn cost_components(&self) -> i32 {
        let mut cost = 0;
        for (id, set) in self.map.iter() {
            for edge in set.iter() {
                cost += edge.cost;
            }
        }
        cost
    }

    fn num_edges(&self) -> usize {
        self.edges
    }
}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut heap = EdgeHeap::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                heap.add((points[i][0], points[i][1]), (points[j][0], points[j][1]));
            }
        }

        let mut components = Components::new();

        while components.num_edges() < points.len() - 1 {
            components.add_edge(heap.pop().unwrap())
        }

        components.cost_components()
    }
}
