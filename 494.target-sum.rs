use std::collections::HashMap;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut old_sums: HashMap<i32, i32> = HashMap::new();
        let mut new_sums: HashMap<i32, i32> = HashMap::new();
        old_sums.insert(0, 1);
        for n in nums.iter() {
            for (i, v) in old_sums.iter() {
                *new_sums.entry(i + n).or_insert(0) += v;
            }
            for (i, v) in old_sums.iter() {
                *new_sums.entry(i - n).or_insert(0) += v;
            }
            old_sums = new_sums;
            new_sums = HashMap::new();
        }
        *old_sums.get(&target).unwrap_or(&0)
    }
}
