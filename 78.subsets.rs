impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut temp: Vec<i32> = Vec::new();

        fn traversal(i: usize, nums: &Vec<i32>, temp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if i >= nums.len() {
                return;
            }
            temp.push(nums[i]);
            res.push(temp.to_vec());
            traversal(i + 1, nums, temp, res);
            temp.pop();
            traversal(i + 1, nums, temp, res);
        }
        traversal(0, &nums, &mut temp, &mut res);
        res.push(Vec::new());
        res
    }
}
