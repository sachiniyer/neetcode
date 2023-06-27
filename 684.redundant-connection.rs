use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Find {
    components: HashMap<i32, HashSet<i32>>,
    ids: VecDeque<i32>,
}

impl Find {
    fn new() -> Self {
        Self {
            components: HashMap::new(),
            ids: {
                let mut v = VecDeque::new();
                v.push_front(0);
                v
            },
        }
    }

    fn id(&mut self) -> i32 {
        let res = self.ids.pop_front().unwrap();
        if self.ids.is_empty() {
            self.ids.push_front(res + 1);
        }
        res
    }

    fn merge(&mut self, set1: i32, set2: i32) {
        let add_set = self
            .components
            .get(&set2)
            .unwrap_or(&HashSet::new())
            .clone();
        (*self.components.entry(set1).or_insert(HashSet::new())).extend(add_set);
        self.components.remove(&set2);
        self.ids.push_front(set2);
    }

    fn insert(&mut self, new: Vec<i32>) -> i32 {
        let id = self.id();
        self.components.insert(id, new.into_iter().collect());
        id
    }

    fn find(&mut self, num: i32) -> Option<i32> {
        for (key, set) in self.components.iter() {
            if set.contains(&num) {
                return Some(*key);
            }
        }
        None
    }

    fn add(&mut self, num: i32, id: i32) {
        match self.find(num) {
            Some(v) => {
                self.merge(id, v);
            }
            None => {
                (*self.components.entry(id).or_insert(HashSet::new())).insert(num);
            }
        }
    }
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut f: Find = Find::new();
        for e in edges {
            let source = e[0];
            let target = e[1];

            match f.find(source) {
                Some(source_id) => match f.find(target) {
                    Some(target_id) => {
                        if source_id == target_id {
                            return e;
                        }
                        f.merge(source_id, target_id);
                    }
                    None => {
                        f.add(target, source_id);
                    }
                },
                None => match f.find(target) {
                    Some(target_id) => {
                        f.add(source, target_id);
                    }
                    None => {
                        f.insert(e);
                    }
                },
            }
        }
        vec![]
    }
}
