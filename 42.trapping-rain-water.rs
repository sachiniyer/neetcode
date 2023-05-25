impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let (top, _) = height
            .iter()
            .enumerate()
            .max_by_key(|(_, &value)| value)
            .unwrap();
        let mut i: usize = 0;
        let mut side: usize = 0;
        let mut sum: i32 = 0;
        while i < top {
            if height[i] >= height[side] {
                side = i;
            } else {
                sum += height[side] - height[i];
            }
            i += 1;
        }
        let mut side: usize = height.len() - 1;
        let mut i: usize = height.len() - 1;
        while i > top {
            if height[i] >= height[side] {
                side = i
            } else {
                sum += height[side] - height[i];
            }
            i -= 1;
        }
        sum
    }
}
