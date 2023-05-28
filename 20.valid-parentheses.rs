use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut map: HashMap<char, char> = HashMap::new();
        map.insert('(', ')');
        map.insert('{', '}');
        map.insert('[', ']');
        for c in s.chars() {
            if let Some(v) = map.get(&c) {
                stack.push(*v);
            } else {
                if stack.is_empty() || c != stack.pop().unwrap() {
                    return false;
                }
            }
        }
        if !stack.is_empty() {
            return false;
        }
        true
    }
}
