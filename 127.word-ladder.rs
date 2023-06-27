// keep_of_kalesin
use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if word_list.iter().find(|&w| end_word.eq(w)).is_none() {
            return 0;
        }

        let mut visited = HashSet::new();
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        let mut next = HashSet::new();

        left.insert(begin_word.as_str());
        right.insert(end_word.as_str());
        visited.insert(begin_word.as_str());
        visited.insert(end_word.as_str());

        let mut len = 1;
        while !left.is_empty() && !right.is_empty() {
            if left.len() > right.len() {
                std::mem::swap(&mut left, &mut right);
            }

            for w in left.drain() {
                for n in word_list.iter().map(|s| s.as_str()) {
                    if Self::is_adjacent(w, n) {
                        if right.contains(n) {
                            return len + 1;
                        }

                        if visited.insert(n) {
                            next.insert(n);
                        }
                    }
                }
            }

            std::mem::swap(&mut left, &mut next);
            len += 1;
        }

        0
    }

    fn is_adjacent(a: &str, b: &str) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();
        assert_eq!(a.len(), b.len());

        let mut diffs = 0;
        for idx in 0..a.len() {
            if a[idx] != b[idx] {
                diffs += 1;
            }

            if diffs > 1 {
                break;
            }
        }

        diffs == 1
    }
}
