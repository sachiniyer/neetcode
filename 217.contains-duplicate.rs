impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut m: HashMap<i32, ()> = HashMap::new();

        for n in nums {
            if m.contains_key(&n) {
                return true;
            }
            m.insert(n, ());
        }
        false
    }
}
