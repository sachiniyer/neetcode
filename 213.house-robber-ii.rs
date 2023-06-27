impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() <= 3 {
            return *nums.iter().max().unwrap();
        }

        let mut i = nums[0];
        let mut j = nums[1];
        for n in 2..nums.len() - 1 {
            let t = j;
            j = i + nums[n];
            i = i.max(t);
        }
        let m1 = i.max(j);

        let mut i = nums[1];
        let mut j = nums[2];
        for n in 3..nums.len() {
            let t = j;
            j = i + nums[n];
            i = i.max(t);
        }
        m1.max(i).max(j)
    }
}
