use std::collections::VecDeque;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: VecDeque<i32> = VecDeque::new();
        for c in tokens {
            let c = match c.as_str() {
                "+" => {
                    let arg2 = stack.pop_front().unwrap();
                    let arg1 = stack.pop_front().unwrap();
                    stack.push_front(arg1 + arg2);
                }
                "-" => {
                    let arg2 = stack.pop_front().unwrap();
                    let arg1 = stack.pop_front().unwrap();
                    stack.push_front(arg1 - arg2);
                }
                "*" => {
                    let arg2 = stack.pop_front().unwrap();
                    let arg1 = stack.pop_front().unwrap();
                    stack.push_front(arg1 * arg2);
                }
                "/" => {
                    let arg2 = stack.pop_front().unwrap();
                    let arg1 = stack.pop_front().unwrap();
                    stack.push_front(arg1 / arg2);
                }

                _ => {
                    stack.push_front(c.parse().unwrap());
                }
            };
        }
        stack.pop_front().unwrap()
    }
}
