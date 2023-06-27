impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let (m, n) = (s.len(), t.len());

        let mut dp = vec![vec![0; n + 1]; m + 1];

        for j in 0..n + 1 {
            dp[m][j] = 0;
        }

        for i in 0..m + 1 {
            dp[i][n] = 1;
        }
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                dp[i][j] = dp[i + 1][j];

                if s[i] == t[j] {
                    dp[i][j] += dp[i + 1][j + 1];
                }
            }
        }
        dp[0][0]
    }
}
