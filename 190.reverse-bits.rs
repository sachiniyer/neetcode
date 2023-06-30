impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;
        let mut pow = 31;
        let mut x = x;
        while x != 0 {
            res += (x & 1) << pow;
            x = x >> 1;
            pow -= 1;
        }
        res
    }
}
