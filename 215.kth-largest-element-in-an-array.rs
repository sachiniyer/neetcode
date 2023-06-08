impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // I am lazy, and actually happy rust was easy for once
        let mut nums = nums;
        *nums
            .select_nth_unstable_by(k as usize - 1, |a, b| b.cmp(a))
            .1
    }
}
