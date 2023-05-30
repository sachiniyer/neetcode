use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut q: VecDeque<(i32, usize)> = VecDeque::new();
        let mut i: usize = (k) as usize;
        for i in (0..(k as usize)) {
            if q.is_empty() || q.get(0).unwrap().0 < nums[i] {
                q.clear();
                q.push_front((nums[i], i))
            } else {
                while q.get(q.len() - 1).unwrap().0 < nums[i] {
                    q.pop_back();
                }
                q.push_back((nums[i], i));
            }
        }
        res.push(q[0].0);
        while i < nums.len() {
            if q.get(0).unwrap().0 < nums[i] {
                q.clear();
                q.push_front((nums[i], i))
            } else {
                while q.get(q.len() - 1).unwrap().0 < nums[i] {
                    q.pop_back();
                }
                q.push_back((nums[i], i));
            }
            while q[0].1 < i - (k as usize) + 1 {
                q.pop_front();
            }
            res.push(q[0].0);
            i += 1;
        }
        res
    }
}
