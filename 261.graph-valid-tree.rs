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
        (*self.components.entry(id).or_insert(HashSet::new())).insert(num);
    }

    fn add_edge(&mut self, source: i32, target: i32) -> bool {
        match self.find(source) {
            Some(source_id) => match self.find(target) {
                Some(target_id) => {
                    if source_id == target_id {
                        return false;
                    }
                    self.merge(source_id, target_id);
                    true
                }
                None => {
                    self.add(target, source_id);
                    true
                }
            },
            None => match self.find(target) {
                Some(target_id) => {
                    self.add(source, target_id);
                    true
                }
                None => {
                    self.insert(vec![source, target]);
                    true
                }
            },
        }
    }

    fn num_components(&self) -> usize {
        self.components.len()
    }

    fn num_nodes(&self) -> usize {
        let mut res = 0;

        for (key, set) in self.components.iter() {
            res += set.len();
        }
        res
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if n == 1 {
            return true;
        }
        let mut f: Find = Find::new();
        for e in edges {
            if !f.add_edge(e[0], e[1]) {
                return false;
            }
        }
        n == f.num_nodes() as i32 && 1 == f.num_components()
    }
}
