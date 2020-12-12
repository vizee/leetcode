impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k as usize > prices.len() / 2 {
            let mut sum = 0;
            for i in 1..prices.len() {
                sum += (prices[i] - prices[i - 1]).max(0);
            }
            return sum;
        }
        let k = (k as usize).min(prices.len() / 2);
        let mut buy = vec![::std::i32::MIN; k];
        let mut sell = vec![0; k];
        for p in prices {
            for i in 0..k {
                buy[i] = buy[i].max(if i > 0 { sell[i - 1] } else { 0 } - p);
                sell[i] = sell[i].max(buy[i] + p);
            }
        }
        *sell.last().unwrap_or(&0)
    }
}
