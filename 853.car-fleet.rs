use std::collections::VecDeque;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut prevec: Vec<(i32, i32, f64)> = Vec::new();
        for i in (0..position.len()) {
            prevec.push((
                position[i],
                speed[i],
                ((target - position[i]) as f64) / (speed[i]) as f64,
            ));
        }
        prevec.sort_by_key(|t| t.0);
        let mut stack: VecDeque<(i32, i32, f64)> = VecDeque::new();

        while prevec.len() > 0 {
            let c = prevec.pop().unwrap();
            if stack.is_empty() || stack.get(0).unwrap().2 < c.2 {
                stack.push_front(c)
            }
        }
        stack.len() as i32
    }
}
