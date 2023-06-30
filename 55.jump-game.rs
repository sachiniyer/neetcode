impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut last = nums.len() as i32 - 1;
        for i in (1..nums.len()).rev() {
            if nums[i] + i as i32 >= last {
                last = i as i32;
            }
        }
        nums[0] >= last
    }
}
