use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

struct TrieNode {
    pub m: HashMap<char, (TrieNode, bool)>,
    pub v: char
}

impl TrieNode {
    pub fn new(v: char) -> Self {
        Self {
            m: HashMap::new(),
            v
        }
    }
}

struct Trie {
    root: TrieNode
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            root: TrieNode::new('-'),
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        let w: Vec<char> = word.chars().collect();
        for s in 0..w.len()-1 {
            let mut entry = node.m.entry(w[s]).or_insert((TrieNode::new(w[s]), false));
            node = &mut entry.0;
        }
        (*node.m.entry(*w.last().unwrap()).or_insert((TrieNode::new(*w.last().unwrap()), true))).1 = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        let w: Vec<char> = word.chars().collect();
        for s in 0..w.len()-1 {
            match node.m.get(&w[s]) {
                Some(n) => {
                    node = &n.0
                },
                None => return false
            }
        }
        match node.m.get(&w.last().unwrap()) {
            Some(n) => {
                n.1
            },
            None => false
        }
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        let w: Vec<char> = prefix.chars().collect();
        for s in 0..w.len()-1 {
            match node.m.get(&w[s]) {
                Some(n) => {
                    node = &n.0
                },
                None => return false
            }
        }
        match node.m.get(&w.last().unwrap()) {
            Some(n) => {
                true
            },
            None => false
        }
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
