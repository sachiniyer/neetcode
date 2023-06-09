use std::collections::HashSet;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut temp: Vec<i32> = Vec::new();
        let mut candidates = candidates.clone();
        candidates.sort();
        fn traversal(temp: &[i32], nums: &[i32], target: &i32, res: &mut Vec<Vec<i32>>) {
            let sum = temp.iter().sum::<i32>();
            if sum == *target {
                res.push(temp.to_vec());
                return;
            } else if sum > *target {
                return;
            }

            let mut seen: HashSet<String> = HashSet::new();
            for (i, v) in nums.iter().enumerate() {
                let mut s = temp.to_vec();
                s.push(*v);
                let sub_new_string: String = s.iter().map(|&x| x.to_string()).collect();

                if !seen.contains(&sub_new_string) {
                    seen.insert(sub_new_string);
                    traversal(&s, &nums[i + 1..], target, res);
                }
            }
        }
        traversal(&temp, &candidates, &target, &mut res);
        res
    }
}
