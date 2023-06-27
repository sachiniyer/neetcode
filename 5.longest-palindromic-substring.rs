impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let mut max: (i32, i32, i32) = (1, 0, 0);
        let length = s.len() as i32;
        for c in 1..s.len() {
            if s[c - 1] == s[c] {
                max = (2, (c - 1) as i32, c as i32);
            }
        }
        for c in 1..s.len() - 1 {
            let mut i = c as i32 - 1;
            let mut j = c as i32 + 1;
            while i >= 0 && j < length {
                if s[i as usize] == s[j as usize] {
                    if j - i + 1 > max.0 {
                        max = (j - i + 1, i, j);
                    }
                } else {
                    break;
                }
                i -= 1;
                j += 1;
            }
        }

        for c in 2..s.len() - 1 {
            if s[c - 1] == s[c] {
                let mut i = c as i32 - 2;
                let mut j = c as i32 + 1;
                while i >= 0 && j < length {
                    if s[i as usize] == s[j as usize] {
                        if j - i + 1 > max.0 {
                            max = (j - i + 1, i, j);
                        }
                    } else {
                        break;
                    }
                    i -= 1;
                    j += 1;
                }
            }
        }
        s[(max.1 as usize)..=(max.2 as usize)].iter().collect()
    }
}
