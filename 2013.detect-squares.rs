use std::collections::HashMap;

struct DetectSquares {
    points: HashMap<Vec<i32>, i32>
}


impl DetectSquares {

    fn new() -> Self {
        Self {
            points: HashMap::new()
        }
    }
    
    fn add(&mut self, point: Vec<i32>) {
        *self.points.entry(point).or_insert(0) += 1;
    }
    
    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        for (k, v) in self.points.iter() {
            if *k != point && (point[0] - k[0]).abs() == (point[1] - k[1]).abs() {
                    res += v * self.points.get(&*vec![k[0], point[1]]).unwrap_or(&0) * self.points.get(&*vec![point[0], k[1]]).unwrap_or(&0);
                }
        }
        res
    }
}

/**
 * Your DetectSquares object will be instantiated and called as such:
 * let obj = DetectSquares::new();
 * obj.add(point);
 * let ret_2: i32 = obj.count(point);
 */
