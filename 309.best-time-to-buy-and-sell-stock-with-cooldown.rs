impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut buy: Vec<i32> = vec![i32::MIN; prices.len()];
        let mut sell: Vec<i32> = vec![i32::MIN; prices.len()];

        buy[0] = -prices[0];
        buy[1] = buy[0].max(-prices[1]);

        sell[0] = 0;
        sell[1] = sell[0].max(buy[0] + prices[1]);

        for i in 2..prices.len() {
            buy[i] = buy[i - 1].max(sell[i - 2] - prices[i]);
            sell[i] = sell[i - 1].max(buy[i - 1] + prices[i]);
        }
        *buy.last().unwrap().max(sell.last().unwrap()).max(&0)
    }
}
