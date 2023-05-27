impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<char, usize> = HashMap::new();
        let mut imap: HashMap<usize, char> = HashMap::new();
        let mut max: i32 = 0;
        let mut l: usize = 0;
        let mut curr: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(value) = map.get(&c) {
                let end = value.clone();
                map.remove(&c);
                imap.remove(&end);
                map.insert(c, i);
                imap.insert(i, c);
                while l < end {
                    if let Some(key) = imap.remove(&l) {
                        println!("{:?}", key);
                        println!("{:?}", l);
                        map.remove(&key);
                    }
                    l += 1;
                }
                l = end;
                curr = (i - l) as i32;
            } else {
                curr += 1;
                map.insert(c, i);
                imap.insert(i, c);
            }
            if curr > max {
                max = curr;
            }
        }
        max
    }
}
