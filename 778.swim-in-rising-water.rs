use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Eq, Clone, Copy)]
struct Cell {
    val: i32,
    y: usize,
    x: usize,
}

impl Cell {
    fn new(val: i32, y: usize, x: usize) -> Self {
        Self { val, y, x }
    }

    fn adjs(&self, n: usize) -> Vec<(usize, usize)> {
        let mut prop = vec![
            (self.y + 1, self.x),
            (self.y - 1, self.x),
            (self.y, self.x + 1),
            (self.y, self.x - 1),
        ];
        let mut i = 0;
        while i < prop.len() {
            if prop[i].0 >= n || prop[i].1 >= n {
                prop.remove(i);
            } else {
                i += 1;
            }
        }
        prop
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        heap.push(Cell::new(grid[0][0], 0, 0));
        let mut visited = HashSet::new();
        let n = grid.len();
        let mut max = 0;
        while let Some(cell) = heap.pop() {
            if visited.contains(&(cell.y, cell.x)) {
                continue;
            }
            visited.insert((cell.y, cell.x));
            max = max.max(cell.val);
            if cell.y == n - 1 && cell.x == n - 1 {
                return max;
            }
            for adj in cell.adjs(n) {
                heap.push(Cell::new(grid[adj.0][adj.1], adj.0, adj.1));
            }
        }
        max
    }
}
