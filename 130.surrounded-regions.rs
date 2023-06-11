impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn traverse(y: usize, x: usize, orig: char, new: char, grid: &mut Vec<Vec<char>>) {
            if grid[y][x] == orig {
                grid[y][x] = new;
                let adjs = vec![(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];
                for a in adjs {
                    if a.0 >= 0 && a.1 >= 0 && a.0 < grid.len() && a.1 < grid[0].len() {
                        if grid[a.0][a.1] == orig {
                            traverse(a.0, a.1, orig, new, grid);
                        }
                    }
                }
            }
        }

        let (rows, cols) = (board.len(), board[0].len());
        let indices: Vec<(usize, usize)> = (0..cols)
            .map(|col| (0, col))
            .chain((0..cols).map(|col| (rows - 1, col)))
            .chain((1..rows - 1).map(|row| (row, 0)))
            .chain((1..rows - 1).map(|row| (row, cols - 1)))
            .collect();

        for i in indices {
            if board[i.0][i.1] == 'O' {
                traverse(i.0, i.1, 'O', 'T', board);
            }
        }

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == 'O' {
                    traverse(y, x, 'O', 'X', board);
                }
            }
        }

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == 'T' {
                    board[y][x] = 'O';
                }
            }
        }
    }
}
