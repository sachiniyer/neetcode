impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.as_bytes()[0] == b'0' {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }
        let mut d1 = 1;

        let mut d2 = 1;
        let mut n = 0;
        for w in s.as_bytes().windows(2) {
            println!("{}, {}", d1, d2);
            n = 0;

            if w[1] != b'0' {
                n += d1;
            }

            if (w[0] == b'1') || (w[0] == b'2' && w[1] - b'0' <= 6) {
                n += d2;
            }

            d2 = d1;
            d1 = n;
        }
        return d1;
    }
}
