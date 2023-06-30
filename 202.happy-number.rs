impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut n = n;

        while n != 1 && n != 4 {
            let mut new = 0;
            while n > 0 {
                let d = n % 10;
                n = n / 10;
                new += d.pow(2)
            }
            n = new;
        }

        n == 1
    }
}
