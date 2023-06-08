impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut temp: Vec<i32> = Vec::new();

        fn traversal(temp: &[i32], nums: &[i32], target: &i32, res: &mut Vec<Vec<i32>>) {
            let sum = temp.iter().sum::<i32>();
            if sum == *target {
                res.push(temp.to_vec());
                return;
            } else if sum > *target {
                return;
            }

            for (i, v) in nums.iter().enumerate() {
                let mut s = temp.to_vec();
                s.push(*v);
                traversal(&s, &nums[i..], target, res);
            }
        }
        traversal(&temp, &candidates, &target, &mut res);
        res
    }
}
