impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];

        for i in 1..(n as usize + 1) {
            res[i] = res[i & (i - 1)] + 1;
        }
        res
    }
}
