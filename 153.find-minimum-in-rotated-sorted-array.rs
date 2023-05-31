impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            if nums[0] < nums[1] {
                return nums[0];
            }
            return nums[1];
        }
        while l + 1 < r {
            let m = ((r - l) / 2) + l;
            if nums[m + 1] > nums[m] && nums[m - 1] > nums[m] {
                return nums[m];
            }
            if (nums[m - 1] < nums[m + 1] && nums[m - 1] < nums[r])
                || (nums[l] < nums[m + 1] && nums[l] < nums[r])
            {
                r = m;
            } else {
                l = m;
            }
        }
        if nums[r] < nums[l] {
            return nums[r];
        }
        nums[l]
    }
}
