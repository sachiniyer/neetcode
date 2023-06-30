impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 1;
        }

        let (mut steps, mut current, mut end) = (0, 0, 0);
        for i in 0..nums.len() - 1 {
            let j = i as i32 + nums[i];
            if current < j {
                current = j;
            }
            if i == end {
                steps += 1;
                end = current as usize;
            }
        }
        steps
    }
}
