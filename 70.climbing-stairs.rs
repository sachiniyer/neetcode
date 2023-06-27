impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a: u32 = 1;
        let mut b: u32 = 1;
        for _ in 0..n {
            let tmp = a;
            a = b;
            b += tmp;
        }
        a as i32
    }
}
