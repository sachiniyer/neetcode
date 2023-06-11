use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut indegree: HashMap<i32, i32> = HashMap::new();
        for i in 0..num_courses {
            graph.insert(i, HashSet::new());
            indegree.insert(i, 0);
        }
        for p in prerequisites {
            graph.get_mut(&p[1]).unwrap().insert(p[0]);
            indegree.insert(p[0], indegree.get(&p[0]).unwrap() + 1);
        }
        let mut queue: VecDeque<i32> = VecDeque::new();
        for (k, v) in indegree.iter() {
            if *v == 0 {
                queue.push_front(*k);
            }
        }
        let mut count = 0;
        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            count += 1;
            for n in graph.get(&node).unwrap() {
                indegree.insert(*n, indegree.get(n).unwrap() - 1);
                if indegree.get(n).unwrap() == &0 {
                    queue.push_front(*n);
                }
            }
        }
        count == num_courses
    }
}
