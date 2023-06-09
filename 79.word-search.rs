use std::collections::VecDeque;
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board.clone();
        let mut word: VecDeque<char> = word.chars().collect();
        fn traversal(x: &usize, y: &usize, word: &VecDeque<char>, board: &Vec<Vec<char>>) -> bool {
            if word.len() == 0 {
                return true;
            }
            let mut word = word.clone();
            let mut board = board.clone();
            let adjs = vec![(*x + 1, *y), (*x - 1, *y), (*x, *y + 1), (*x, *y - 1)];
            let match_char = word.pop_front().unwrap();
            for i in adjs {
                if i.0 < board.len() && i.1 < board[0].len() {
                    if match_char == board[i.0][i.1] {
                        board[i.0][i.1] = '#';
                        if traversal(&i.0, &i.1, &word, &board) {
                            return true;
                        }
                        board[i.0][i.1] = match_char;
                    }
                }
            }
            false
        }

        let match_char = word.pop_front().unwrap();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == match_char {
                    board[i][j] = '#';
                    if traversal(&i, &j, &word, &board) {
                        return true;
                    }
                    board[i][j] = match_char;
                }
            }
        }
        false
    }
}
