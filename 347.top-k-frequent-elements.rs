impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            *m.entry(n).or_insert(0) += 1
        }

        let mut a: Vec<Vec<i32>> = vec![vec![]; 1000];
        for (k, v) in &m {
            a[*v as usize].push(*k);
        }
        let mut ret: Vec<i32> = Vec::new();
        for i in (0..1000).rev() {
            if !a[i].is_empty() {
                ret.extend_from_slice(&a[i]);
            }
            if ret.len() as i32 == k {
                return ret;
            }
        }
        ret
    }
}
