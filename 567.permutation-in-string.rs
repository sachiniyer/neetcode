use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map: HashMap<char, i32> = HashMap::new();
        use std::collections::VecDeque;
        let mut removed: VecDeque<char> = VecDeque::new();
        let mut chars_left = s1.len();
        for c in s1.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for c in s2.chars() {
            if let Some(v) = map.get_mut(&c) {
                if *v == 0 {
                    let mut stop = false;

                    let mut last = removed.pop_back().unwrap();
                    while last != c {
                        *map.get_mut(&last).unwrap() += 1;
                        chars_left += 1;
                        last = removed.pop_back().unwrap();
                    }
                    removed.push_front(c);
                } else {
                    *v -= 1;
                    chars_left -= 1;
                    if chars_left == 0 {
                        return true;
                    }
                    removed.push_front(c);
                }
            } else {
                while removed.len() > 0 {
                    *map.get_mut(&removed.pop_back().unwrap()).unwrap() += 1;
                }
                chars_left = s1.len();
            }
        }
        false
    }
}
