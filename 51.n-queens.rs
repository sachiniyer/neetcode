// sugyan

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut answer = Vec::new();
        let mut v = Vec::new();
        Self::backtrack(n, &mut v, &mut answer);
        answer
    }
    fn backtrack(n: i32, v: &mut Vec<i32>, answer: &mut Vec<Vec<String>>) {
        if v.len() == n as usize {
            answer.push(
                v.iter()
                    .map(|&p| (0..n).map(|i| if i == p { 'Q' } else { '.' }).collect())
                    .collect(),
            );
            return;
        }
        for i in 0..n {
            if v.iter()
                .enumerate()
                .any(|(j, &p)| p == i || (v.len() - j) as i32 == (p - i).abs())
            {
                continue;
            }
            v.push(i);
            Self::backtrack(n, v, answer);
            v.pop();
        }
    }
}
