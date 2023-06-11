impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid.clone();
        let mut res = 0;

        fn traverse(x: usize, y: usize, grid: &mut Vec<Vec<i32>>, res: &mut i32) {
            if grid[x][y] == 1 {
                grid[x][y] = 0;
                let adjs = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
                for a in adjs {
                    if a.0 >= 0 && a.1 >= 0 && a.0 < grid.len() && a.1 < grid[0].len() {
                        if grid[a.0][a.1] == 1 {
                            *res += 1;
                            traverse(a.0, a.1, grid, res);
                        }
                    }
                }
            }
        }

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == 1 {
                    let mut new_res = 1;
                    traverse(x, y, &mut grid, &mut new_res);
                    res = res.max(new_res);
                }
            }
        }

        res
    }
}
