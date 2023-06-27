use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp: Vec<bool> = vec![false; s.len() + 1];
        let set: HashSet<String> = word_dict.into_iter().collect();
        dp[0] = true;

        for i in 0..s.len() {
            for j in (0..=i).rev() {
                if set.contains(&s[j..i + 1]) && dp[j] {
                    dp[i + 1] = true;
                    break;
                }
            }
        }

        dp[s.len()]
    }
}
