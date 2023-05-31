impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // find row
        let mut row = 0;
        if matrix.len() == 2 {
            if matrix[1][0] <= target {
                row = 1;
            }
        }
        let mut l: usize = 0;
        let mut r: usize = matrix.len() - 1;
        while l + 1 < r {
            let m = ((r - l) / 2) + l;
            if matrix[m][0] <= target {
                l = m;
            } else {
                r = m;
            }
        }
        if l != r {
            if matrix[r][0] <= target {
                row = r;
            } else {
                row = l;
            }
        } else {
            row = l;
        }

        // find item
        l = 0;
        r = matrix[row].len() - 1;

        if matrix[row].len() == 1 {
            if matrix[row][0] == target {
                return true;
            }
            return false;
        }

        if matrix[row].len() == 2 {
            if matrix[row][0] == target {
                return true;
            }
            if matrix[row][1] == target {
                return true;
            }
            return false;
        }
        while l + 1 < r {
            let m = ((r - l) / 2) + l;
            if matrix[row][m] < target {
                l = m;
            } else {
                r = m;
            }
            if matrix[row][r] == target {
                return true;
            }
            if matrix[row][l] == target {
                return true;
            }
        }
        return false;
    }
}
