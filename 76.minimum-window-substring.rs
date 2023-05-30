use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut w: (usize, usize) = (0, s.len());
        let mut l: usize = 0;
        let mut r: usize = 0;
        let schars: Vec<char> = s.chars().collect();

        for c in t.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        let mut charset: HashSet<char> = map.keys().cloned().collect();

        while r < s.len() {
            if let Some(v) = map.get_mut(&schars[r]) {
                *v -= 1;
                if *v <= 0 {
                    charset.remove(&schars[r]);
                }
            }
            let mut cont = true;
            while cont && l < r {
                if let Some(v) = map.get_mut(&schars[l]) {
                    if *v >= 0 {
                        cont = false;
                    } else {
                        *v += 1;
                        l += 1;
                    }
                } else {
                    l += 1;
                }
            }
            if charset.len() == 0 {
                if r - l < w.1 - w.0 {
                    w = (l, r);
                }
            }
            r += 1;
        }
        if w.1 - w.0 == s.len() {
            return "".to_string();
        }
        s[w.0..(w.1 + 1)].to_string()
    }
}
