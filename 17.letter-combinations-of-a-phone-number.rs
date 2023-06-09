use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut digit_map: HashMap<char, Vec<char>> = HashMap::new();
        // there are better ways to do this, but I am too braindead to implement
        digit_map.insert('2', vec!['a', 'b', 'c']);
        digit_map.insert('3', vec!['d', 'e', 'f']);
        digit_map.insert('4', vec!['g', 'h', 'i']);
        digit_map.insert('5', vec!['j', 'k', 'l']);
        digit_map.insert('6', vec!['m', 'n', 'o']);
        digit_map.insert('7', vec!['p', 'q', 'r', 's']);
        digit_map.insert('8', vec!['t', 'u', 'v']);
        digit_map.insert('9', vec!['w', 'x', 'y', 'z']);

        let digits_vec: Vec<char> = digits.chars().collect();
        let mut digits: VecDeque<Vec<char>> = VecDeque::new();
        for d in digits_vec {
            digits.push_front((*digit_map.get(&d).unwrap()).to_vec());
        }

        fn traverse(sub: &Vec<char>, digits: &VecDeque<Vec<char>>, res: &mut Vec<String>) {
            if digits.len() == 0 {
                res.push(sub.iter().rev().collect());
                return;
            }

            let mut sub = sub.to_vec();
            let mut digits = digits.clone();
            let choices = digits.pop_front().unwrap();

            for c in choices {
                sub.push(c);
                traverse(&sub, &digits, res);
                sub.pop();
            }
        }
        let mut res: Vec<String> = Vec::new();
        let sub: Vec<char> = Vec::new();
        traverse(&sub, &digits, &mut res);
        res
    }
}
