impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            }
            return -1;
        }
        if nums.len() == 2 {
            if nums[0] == target {
                return 0;
            }
            if nums[1] == target {
                return 1;
            }
            return -1;
        }
        while l + 1 < r {
            let m = ((r - l) / 2) + l;
            if ((nums[m - 1] - target).abs() < (nums[m + 1] - target).abs()
                && (nums[m - 1] - target).abs() < (nums[r] - target).abs())
                || ((nums[l] - target).abs() < (nums[m + 1] - target).abs()
                    && (nums[l] - target).abs() < (nums[r] - target).abs())
            {
                r = m;
            } else {
                l = m;
            }
            if nums[r] == target {
                return r as i32;
            }
            if nums[l] == target {
                return l as i32;
            }
        }
        -1
    }
}
