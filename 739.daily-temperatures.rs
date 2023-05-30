use std::collections::VecDeque;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: VecDeque<(i32, usize)> = VecDeque::new();
        let mut res: Vec<i32> = vec![0; temperatures.len()];
        if temperatures.len() == 0 {
            return vec![];
        }
        stack.push_front((temperatures[0], 0));
        for t in (1..temperatures.len()) {
            while !stack.is_empty() && temperatures[t] > stack[0].0 {
                let out = stack.pop_front().unwrap();
                res[out.1] = (t - out.1) as i32;
            }
            stack.push_front((temperatures[t], t));
        }
        res
    }
}
