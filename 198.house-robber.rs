impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut i = nums[0];
        let mut j = nums[1];
        for n in 2..nums.len() {
            let t = j;
            j = i + nums[n];
            i = i.max(t);
        }
        i.max(j)
    }
}
