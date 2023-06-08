use itertools::Itertools;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.iter().permutations(nums.len())
    }
}
