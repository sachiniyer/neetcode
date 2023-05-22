impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut s: HashSet<i32> = HashSet::new();

        for n in nums {
            s.insert(n);
        }

        let mut max: i32 = 0;
        let mut start: i32 = 0;
        let mut end: i32 = 0;
        while s.len() != 0 {
            if let Some(e) = s.iter().next().cloned() {
                s.remove(&e);
                start = e;
                end = e;
                while s.contains(&(start - 1)) {
                    start -= 1;
                    s.remove(&start);
                }
                while s.contains(&(end + 1)) {
                    end += 1;
                    s.remove(&end);
                }
                if end - start + 1 > max {
                    max = end - start + 1;
                }
            }
        }
        max
    }
}
