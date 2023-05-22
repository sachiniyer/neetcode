impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut pre: i32 = 1;
        let mut post: i32 = 1;

        let mut ret: Vec<i32> = Vec::new();
        for n in &nums {
            ret.push(pre);
            pre *= *n;
        }

        for i in (0..ret.len()).rev() {
            ret[i] = ret[i] * post;
            post *= nums[i];
        }

        ret
    }
}
