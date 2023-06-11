use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut rotten: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh: i32 = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == 2 {
                    rotten.push_front((y, x));
                } else if grid[y][x] == 1 {
                    fresh += 1;
                }
            }
        }
        if fresh <= 0 {
            return 0;
        }
        let mut res = 0;
        while rotten.len() > 0 {
            let mut children: VecDeque<(usize, usize)> = VecDeque::new();
            while rotten.len() > 0 {
                let (y, x) = rotten.pop_front().unwrap();

                let adjs = vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

                for a in adjs {
                    if a.0 >= 0 && a.1 >= 0 && a.0 < grid.len() && a.1 < grid[0].len() {
                        if grid[a.0][a.1] == 1 {
                            grid[a.0][a.1] = 2;
                            fresh -= 1;
                            children.push_front((a.0, a.1));
                        }
                    }
                }
            }

            rotten = children;

            if rotten.len() > 0 {
                res += 1;
            }
        }
        if fresh > 0 {
            return -1;
        }
        res
    }
}
