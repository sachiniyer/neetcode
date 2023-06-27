impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; nums.len() + 1];
        let mut res = 1;
        for i in (0..=dp.len() - 1).rev() {
            let mut max = 1;

            for j in (i)..nums.len() {
                if nums[j] > nums[i] {
                    max = max.max(1 + dp[j]);
                }
            }
            dp[i] = max;
            res = res.max(max);
        }

        res
    }
}
