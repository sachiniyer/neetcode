impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut min, mut max) = (0, 0);
        for s in s.chars() {
            match s {
                s if s == '(' => {
                    min += 1;
                    max += 1
                }
                s if s == '*' => {
                    max += 1;
                    min -= 1;
                }
                s if s == ')' => {
                    max -= 1;
                    min -= 1;
                }
                _ => unreachable!(),
            }
            if (max < 0) {
                return false;
            };
            min = min.max(0);
        }
        min == 0
    }
}
