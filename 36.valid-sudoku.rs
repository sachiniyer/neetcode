impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        /// 1-3, 1-3; 1-3, 4-6,
        use std::collections::HashMap;
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let mut m: HashMap<char, bool> = HashMap::new();
                let v = vec![
                    (i, j),
                    (i, j + 1),
                    (i, j + 2),
                    (i + 1, j),
                    (i + 1, j + 1),
                    (i + 1, j + 2),
                    (i + 2, j),
                    (i + 2, j + 1),
                    (i + 2, j + 2),
                ];
                for k in v {
                    if board[k.0][k.1] != '.' && m.contains_key(&board[k.0][k.1]) {
                        return false;
                    }
                    m.insert(board[k.0][k.1].clone(), true);
                }
            }
        }

        for i in 0..9 {
            let mut m1: HashMap<char, bool> = HashMap::new();
            let mut m2: HashMap<char, bool> = HashMap::new();
            for j in 0..9 {
                if board[i][j] != '.' && m1.contains_key(&board[i][j]) {
                    return false;
                }
                if board[i][j] != '.' {
                    m1.insert(board[i][j].clone(), true);
                }

                if board[j][i] != '.' && m2.contains_key(&board[j][i]) {
                    return false;
                }
                if board[j][i] != '.' {
                    m2.insert(board[j][i].clone(), true);
                }
            }
        }
        true
    }
}
