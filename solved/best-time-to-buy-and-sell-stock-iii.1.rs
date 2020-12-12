impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let k = 2;
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
