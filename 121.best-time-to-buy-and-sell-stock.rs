impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut m: i32 = 0;
        let mut l: usize = 0;
        let mut r: usize = 1;
        while r < prices.len() {
            if prices[r] - prices[l] > m {
                m = prices[r] - prices[l];
            }
            if prices[r] < prices[l] {
                l = r;
            }
            r += 1;
        }
        m
    }
}
