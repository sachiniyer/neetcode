impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut min = 0;
        let mut res = max;

        for i in nums.iter().skip(1) {
            let i = *i;
            let max_save = max;
            max = (max * i).max(min * i).max(i);
            min = (max_save * i).min(min * i).min(i);
            res = res.max(max);
        }
        res
    }
}
