use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut maxf: i32 = 0;
        let mut res: i32 = 0;
        let s: Vec<char> = s.chars().collect();
        while r < s.len() {
            *map.entry(s[r]).or_default() += 1;
            maxf = maxf.max(*map.get(&s[r]).unwrap());

            while (r - l + 1) as i32 - maxf > k {
                *map.get_mut(&s[l]).unwrap() -= 1;
                l += 1;
            }

            res = res.max((r - l + 1) as i32);
            r += 1;
        }
        res
    }
}
