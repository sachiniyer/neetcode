/**
 * I didn't do this well, but I got too lazy to write my own
 * Linked List implementation. That stuff, especially is Rust is hard
 *  */

use std::collections::VecDeque;
use std::collections::HashMap;

struct LRUCache {
    s: VecDeque<i32>,
    m: HashMap<i32, i32>,
    c: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        Self {
            s: VecDeque::new(),
            m: HashMap::new(),
            c: capacity as usize,
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if self.m.contains_key(&key) {
            let i = self.s.iter().position(|&x| x == key).unwrap();
            self.s.remove(i);
            self.s.push_front(key);
            return self.m.get(&key).cloned().unwrap();
        }
        -1
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.m.contains_key(&key) {
            let i = self.s.iter().position(|&x| x == key).unwrap();
            self.s.remove(i);
            self.s.push_front(key);
            self.m.insert(key, value);
        }
        else {
            if self.m.len() >= self.c {
                let last = self.s.pop_back().unwrap();
                self.m.remove(&last);
            }
            self.m.insert(key, value);
            self.s.push_front(key);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
