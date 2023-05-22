impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m: HashMap<i32, i32> = HashMap::new();

        for (i, e) in nums.iter().enumerate() {
            if let Some(v) = m.get(&(target - e)) {
                return vec![*v, i as i32];
            }

            m.insert(*e, i as i32).unwrap_or_default();
        }

        return vec![];
    }
}
