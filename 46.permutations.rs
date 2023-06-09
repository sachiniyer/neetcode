impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let sub: Vec<i32> = Vec::new();
        let avail = nums.clone();

        fn traversal(sub: &[i32], avail: &[i32], res: &mut Vec<Vec<i32>>) {
            if (avail.len() == 0) {
                res.push(sub.to_vec());
            }
            for (i, v) in avail.iter().enumerate() {
                let mut sub_new = sub.to_vec();
                let mut avail_new = avail.to_vec();
                avail_new.remove(i as usize);
                sub_new.push(*v);
                traversal(&sub_new, &avail_new, res);
            }
        }

        traversal(&sub, &avail, &mut res);
        res
    }
}
