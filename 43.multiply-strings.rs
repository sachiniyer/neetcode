impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();
        let mut digits = vec![0; n + m];
        for (i, c1) in num1.chars().rev().enumerate() {
            for (j, c2) in num2.chars().rev().enumerate() {
                let n1 = c1 as i32 - '0' as i32;
                let n2 = c2 as i32 - '0' as i32;
                let res = n1 * n2 + digits[i + j];
                digits[i + j] = res % 10;
                digits[i + j + 1] += res / 10;
            }
        }
        while digits.len() > 1 && digits.last().unwrap() == &0 {
            digits.pop();
        }
        digits.into_iter().rev().map(|d| d.to_string()).collect()
    }
}
