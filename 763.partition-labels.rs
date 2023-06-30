use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut map = HashMap::new();
        for letter in b'a'..=b'z' {
            let letter_char = letter as char;

            if let Some(end) = s.rfind(letter_char) {
                map.insert(letter_char, end);
            }
        }

        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut res = vec![];
        let mut i = 0;
        while start < s.len() {
            let mut end = *map.get(&s[start]).unwrap();
            while i < end {
                i = i + 1;
                end = end.max(*map.get(&s[i]).unwrap());
            }
            res.push((end - start + 1) as i32);
            start = end + 1;
        }
        res
    }
}
