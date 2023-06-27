use std::collections::HashSet;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let target: i32 = nums.iter().sum();
        if target % 2 == 1 {
            return false;
        }
        let target = target / 2;
        let mut set: HashSet<i32> = HashSet::new();
        set.insert(0);
        for i in nums.iter() {
            let old_set: Vec<i32> = set.iter().map(|x| *x).collect();
            for j in old_set {
                if i + j == target {
                    return true;
                }
                set.insert(i + j);
            }
        }
        false
    }
}
