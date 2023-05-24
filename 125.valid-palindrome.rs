impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut sc: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
        sc = sc.to_lowercase();
        let sc: Vec<char> = sc.chars().collect();
        let lh = ((sc.len() / 2) as f64).ceil() as usize;
        for i in 0..lh {
            if (sc[i] != sc[sc.len() - i - 1]) {
                return false;
            }
        }
        true
    }
}
