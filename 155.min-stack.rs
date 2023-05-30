use std::collections::VecDeque;

struct MinStack {
    stack: VecDeque<i32>,
    mins: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: VecDeque::new(),
            mins: VecDeque::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack.push_front(val);
        if self.mins.is_empty() || val <= *self.mins.get(0).unwrap() {
           self.mins.push_front(*self.stack.get(0).unwrap());
        }
    }
    
    fn pop(&mut self) {
        if self.mins.get(0).unwrap() == self.stack.get(0).unwrap() {
            self.mins.pop_front();
        }
        self.stack.pop_front();
    }
    
    fn top(&self) -> i32 {
        *self.stack.get(0).unwrap()
    }
    
    fn get_min(&self) -> i32 {
        *self.mins.get(0).unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
