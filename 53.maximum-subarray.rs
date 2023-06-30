impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut curr = nums[0];
        for i in nums.iter().skip(1) {
            let full = i + curr;
            curr = full.max(*i);
            max = max.max(curr);
        }
        max
    }
}
