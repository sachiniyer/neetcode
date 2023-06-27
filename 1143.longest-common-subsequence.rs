impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; text1.len() + 1]; text2.len() + 1];

        for (prev_row, letter2) in text2.iter().enumerate() {
            let row = prev_row + 1;

            for (prev_column, letter1) in text1.iter().enumerate() {
                let column = prev_column + 1;

                let mut new = 0;
                if letter1 == letter2 {
                    new = dp[prev_row][prev_column] + 1;
                }
                dp[row][column] = dp[prev_row][column].max(dp[row][prev_column]).max(new);
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}
