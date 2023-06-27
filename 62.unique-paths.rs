use std::cmp;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // too lazy to impl factorial, so copy someone
        let big = (cmp::max(m, n) - 1) as u128;
        let small = (cmp::min(m, n) - 1) as u128;
        ((big + 1..=big + small).product::<u128>() / ((1..=small).product::<u128>())) as i32
    }
}
