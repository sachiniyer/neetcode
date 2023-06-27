impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut s: Vec<char> = s.chars().collect();
        let length = s.len() as i32;
        let mut count = 0;
        for c in 0..s.len() {
            count += 1;
            let mut i = c as i32 - 1;
            let mut j = c as i32 + 1;
            while i >= 0 && j < length {
                if s[i as usize] == s[j as usize] {
                    count += 1;
                } else {
                    break;
                }
                i -= 1;
                j += 1;
            }
        }

        for c in 1..s.len() {
            if s[c - 1] == s[c] {
                count += 1;
                let mut i = c as i32 - 2;
                let mut j = c as i32 + 1;
                while i >= 0 && j < length {
                    if s[i as usize] == s[j as usize] {
                        count += 1;
                    } else {
                        break;
                    }
                    i -= 1;
                    j += 1;
                }
            }
        }
        count
    }
}
