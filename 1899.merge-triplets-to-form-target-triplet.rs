impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut stack = vec![];
        let (v1, v2, v3) = (target[0], target[1], target[2]);
        let (mut e1, mut e2, mut e3) = (false, false, false);
        for arr in triplets {
            if v1 >= arr[0] && v2 >= arr[1] && v3 >= arr[2] {
                e1 |= v1 == arr[0];
                e2 |= v2 == arr[1];
                e3 |= v3 == arr[2];
                stack.push(arr);
            }
        }
        if !e1 || !e2 || !e3 {
            return false;
        }
        !stack.is_empty()
    }
}
