use std::collections::VecDeque;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_palindrome(s: &String) -> bool {
            s.chars().eq(s.chars().rev())
        }

        fn traversal(sub: &Vec<String>, word: &VecDeque<char>, res: &mut Vec<Vec<String>>) {
            if word.len() == 0 {
                res.push(sub.to_vec());
            }
            let mut word = word.clone();
            let mut sub = sub.to_vec();
            let mut new_string: Vec<char> = Vec::new();
            while word.len() > 0 {
                new_string.push(word.pop_front().unwrap());
                if is_palindrome(&new_string.iter().collect()) {
                    sub.push(new_string.iter().collect());
                    traversal(&sub, &word, res);
                    sub.pop();
                }
            }
        }
        let mut res: Vec<Vec<String>> = Vec::new();
        let sub: Vec<String> = Vec::new();
        let mut word: VecDeque<char> = s.chars().collect();

        traversal(&sub, &word, &mut res);
        res
    }
}
