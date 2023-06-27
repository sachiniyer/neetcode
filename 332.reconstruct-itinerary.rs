use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Eq, Clone, PartialEq)]
struct Target(Vec<char>);

impl Ord for Target {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.0.len() {
            if self.0[i] < other.0[i] {
                return Ordering::Less;
            }
            if self.0[i] > other.0[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Airport {
    pub name: Vec<char>,
    pub targets_set: HashSet<Vec<char>>,
    pub targets_heap: BinaryHeap<Target>,
}

impl Airport {
    fn new(name_str: String) -> Self {
        let name: Vec<char> = name_str.chars().collect();
        Self {
            name,
            targets_set: HashSet::new(),
            targets_heap: BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, t: Vec<char>) {
        self.targets_set.insert(t.clone());
        self.targets_heap.push(Target(t.clone()));
    }

    pub fn remove(&mut self, t: Vec<char>) {
        self.targets_set.remove(&t);
        let mut stack: VecDeque<Target> = VecDeque::new();
        while let Some(i) = self.targets_heap.pop() {
            if i.0 == t {
                break;
            }
            stack.push_back(i);
        }
        while let Some(i) = stack.pop_back() {
            self.targets_heap.push(i);
        }
    }

    pub fn contains(&self, t: Vec<char>) -> bool {
        self.targets_set.contains(&t)
    }
}

struct Airports {
    set: HashMap<Vec<char>, Rc<RefCell<Airport>>>,
}

impl Airports {
    fn new() -> Self {
        Self {
            set: HashMap::new(),
        }
    }

    fn add_airport(&mut self, airport: String) {
        if self
            .set
            .contains_key(&airport.chars().collect::<Vec<char>>())
        {
            return;
        }
        let mut t = Rc::new(RefCell::new(Airport::new(airport)));
        let name = t.borrow().name.clone();
        self.set.insert(name, t);
    }

    fn add_flight(&mut self, src: String, dest: String) {
        self.add_airport(src.clone());
        self.add_airport(dest.clone());

        let src = self.set.get(&src.chars().collect::<Vec<char>>()).unwrap();
        src.borrow_mut().add(dest.chars().collect::<Vec<char>>());
    }

    fn dfs(&self, src: Vec<char>, min: i32, sub: Vec<Vec<char>>) -> Vec<Vec<char>> {
        if min == 0 {
            return sub;
        }
        let src = self.set.get(&src).unwrap();
        let targets = src.borrow().targets_heap.clone().into_sorted_vec();
        if targets.len() == 0 {
            return vec![];
        }
        for i in targets {
            let mut sub = sub.clone();
            sub.push(i.0.clone());
            src.borrow_mut().remove(i.0.clone());
            let res = self.dfs(i.0.clone(), min - 1, sub);
            if res.len() > 0 {
                return res;
            }
            src.borrow_mut().add(i.0.clone());
        }
        return vec![];
    }

    fn traverse(&mut self, src: String, min: i32) -> Vec<Vec<char>> {
        self.dfs(src.chars().collect(), min, vec![src.chars().collect()])
    }
}

fn lexical_order(a: Vec<char>, b: Vec<char>) -> bool {
    for i in 0..a.len() {
        if a[i] < b[i] {
            return true;
        }
        if a[i] > b[i] {
            return false;
        }
    }
    false
}

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut airports = Airports::new();
        let ticket_len = tickets.len() as i32;
        for i in tickets {
            let src = i[0].clone();
            let dest = i[1].clone();
            airports.add_flight(src, dest);
        }
        airports
            .traverse("JFK".to_string(), ticket_len)
            .iter()
            .map(|i| i.iter().collect::<String>())
            .collect()
    }
}
