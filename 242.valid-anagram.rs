impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut m: HashMap<char, u32> = HashMap::new();
        if s.len() != t.len() {
            return false;
        }

        for ch in s.chars() {
            *m.entry(ch).or_insert(1) += 1;
        }
        for ch in t.chars() {
            if let Some(v) = m.get_mut(&ch) {
                *v -= 1;
                if *v <= 0 {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}
