impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        use std::iter::FromIterator;
        let mut r: HashMap<String, Vec<String>> = HashMap::new();

        for su in strs {
            let mut chars: Vec<char> = su.chars().collect();
            chars.sort_unstable();
            let ss = String::from_iter(chars);
            r.entry(ss).or_insert(Vec::new()).push(su);
        }

        r.values().cloned().collect()
    }
}
