use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut indegree: HashMap<i32, i32> = HashMap::new();
        for i in 0..num_courses {
            graph.insert(i, HashSet::new());
            indegree.insert(i, 0);
        }
        for p in prerequisites {
            graph.get_mut(&p[1]).unwrap().insert(p[0]);
            *indegree.entry(p[0]).or_insert(0) += 1;
        }
        let mut queue: VecDeque<i32> = VecDeque::new();
        for (k, v) in indegree.iter() {
            if *v == 0 {
                queue.push_front(*k);
            }
        }
        let mut count = 0;
        let mut res: Vec<i32> = Vec::new();
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            res.push(node);
            count += 1;
            for n in graph.get(&node).unwrap() {
                indegree.insert(*n, indegree.get(n).unwrap() - 1);
                if indegree.get(n).unwrap() == &0 {
                    queue.push_front(*n);
                }
            }
        }
        if count == num_courses {
            return res;
        }
        vec![]
    }
}
