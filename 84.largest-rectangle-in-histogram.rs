use std::collections::VecDeque;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: VecDeque<(i32, usize)> = VecDeque::new();
        let mut max: i32 = 0;
        for (i, v) in heights.iter().enumerate() {
            while !stack.is_empty() && *v < stack.get(0).unwrap().0 {
                let col = stack.pop_front().unwrap();
                if stack.is_empty() {
                    let area = (i as i32) * col.0;
                    if area > max {
                        max = area;
                    }
                } else {
                    let width = i - stack.get(0).unwrap().1 - 1;
                    let area = (width as i32) * col.0;
                    if area > max {
                        max = area;
                    }
                }
            }

            if stack.is_empty() {
                stack.push_front((*v, i));
            } else {
                if stack.get(0).unwrap().0 != *v {
                    stack.push_front((*v, i));
                } else {
                    stack.pop_front();
                    stack.push_front((*v, i));
                }
            }
        }
        while !stack.is_empty() {
            let col = stack.pop_front().unwrap();
            if stack.is_empty() {
                let area = (heights.len() as i32) * col.0;
                if area > max {
                    max = area;
                }
            } else {
                let width = heights.len() - col.1 + (col.1 - stack.get(0).unwrap().1 - 1);
                let area = (width as i32) * col.0;
                if area > max {
                    max = area;
                }
            }
        }
        max
    }
}
