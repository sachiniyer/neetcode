impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut i: i32 = nums[0];
        let mut j: i32 = nums[0];

        loop {
            i = nums[i as usize];
            j = nums[nums[j as usize] as usize];
            if i == j {
                i = nums[0];
                while i != j {
                    i = nums[i as usize];
                    j = nums[j as usize];
                }
                return j;
            }
        }
    }
}
