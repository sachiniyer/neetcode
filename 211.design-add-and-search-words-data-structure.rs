use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::VecDeque;

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

struct WordDictionary {
    root: TrieNode

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
         Self {
            root: TrieNode::new('-'),
        }
       
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        let w: Vec<char> = word.chars().collect();
        for s in 0..w.len()-1 {
            let mut entry = node.m.entry(w[s]).or_insert((TrieNode::new(w[s]), false));
            node = &mut entry.0;
        }
        (*node.m.entry(*w.last().unwrap()).or_insert((TrieNode::new(*w.last().unwrap()), true))).1 = true;
    }
    
    fn search(&mut self, word: String) -> bool {
        fn search_stack(node: &TrieNode, word_stack: &mut VecDeque<char>) -> bool {
            let wp = word_stack.pop_front();
            match wp {
                Some(w) => {
                    match w {
                        '.' => {
                            if word_stack.is_empty() {
                                return node.m.values().any(|v| v.1);
                            }
                            node.m.values().any(|v| search_stack(&v.0, &mut word_stack.clone()))
                        }
                        _ => {
                            match node.m.get(&w) {
                                Some(n) => {
                                    if word_stack.is_empty(){
                                        return n.1;
                                    }
                                    search_stack(&n.0, word_stack)
                                },
                                None => false
                            }
                        }
                    }
                },
                None => true

            }
        }
        search_stack(&mut self.root, &mut word.chars().collect::<VecDeque<char>>())
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
