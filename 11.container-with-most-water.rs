impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max = 0;
        while l < r {
            let try_max = (r - l) as i32 * std::cmp::min(height[l], height[r]);
            max = std::cmp::max(try_max, max);
            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }

        max
    }
}
