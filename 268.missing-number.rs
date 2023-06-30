impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res = nums.len() as i32;
        for (i, n) in nums.iter().enumerate() {
            res ^= i as i32 ^ *n;
        }
        res
    }
}
