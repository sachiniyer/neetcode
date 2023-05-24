impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        let mut res: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable();
        let mut target = 0;
        let mut i = 0;
        let mut first = true;
        for i in 0..nums.len() {
            let a = nums[i];
            if i > 0 && a == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = a + nums[l] + nums[r];
                if sum < 0 {
                    l += 1;
                } else if sum > 0 {
                    r -= 1
                } else {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    while nums[l] == nums[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }
        res
    }
}
