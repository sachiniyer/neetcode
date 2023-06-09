use std::collections::HashSet;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut sub: Vec<i32> = Vec::new();
        let mut nums = nums.clone();
        nums.sort();
        fn traversal(sub: &[i32], nums: &[i32], res: &mut Vec<Vec<i32>>) {
            res.push(sub.to_vec());
            let mut seen: HashSet<String> = HashSet::new();
            for (i, v) in nums.iter().enumerate() {
                let mut sub_new = sub.to_vec();
                let mut nums_new = nums[i + 1..].to_vec();
                sub_new.push(*v);
                let sub_new_string: String = sub_new.iter().map(|&x| x.to_string()).collect();
                if !seen.contains(&sub_new_string) {
                    seen.insert(sub_new_string);
                    traversal(&sub_new, &nums_new, res);
                }
            }
        }
        traversal(&sub, &nums, &mut res);
        res
    }
}
