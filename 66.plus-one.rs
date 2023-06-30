use std::collections::VecDeque;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits: VecDeque<i32> = digits.into_iter().collect();
        digits.push_front(0);
        let mut carry = true;
        let mut i = digits.len() - 1;
        while carry {
            if digits[i] == 9 {
                digits[i] = 0;
                i -= 1;
            } else {
                digits[i] += 1;
                carry = false
            }
        }
        if digits[0] == 0 {
            digits.pop_front();
        }
        digits.into_iter().collect()
    }
}
