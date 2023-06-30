impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col = false;
        let n = matrix.len();
        let m = matrix[0].len();

        for i in 0..n {
            if matrix[i][0] == 0 {
                col = true;
            }
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..n {
            for j in 1..m {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }

        if col {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}
