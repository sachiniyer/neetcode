impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                match grid[i][j] {
                    '1' => {
                        let adjs = vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)];
                        let mut island = true;
                        let mut a = 0;
                        while island && a < 4 {
                            if adjs[a].0 < grid.len() && adjs[a].1 < grid[0].len() {
                                match grid[adjs[a].0][adjs[a].1] {
                                    '0' => island = false,
                                    _ => (),
                                }
                            }
                            a += 1;
                        }
                        if island {
                            res += 1;
                        }
                    }
                    _ => (),
                }
            }
        }

        res
    }
}
