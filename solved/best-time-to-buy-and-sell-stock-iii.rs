impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut maxprofit = 0;
        let mut profit = Vec::new();
        profit.resize(prices.len(), 0);
        for i in 1..prices.len() {
            profit[i] = profit[i - 1];
            for j in 0..i {
                let cur = (prices[i] - prices[j]).max(0);
                if cur > profit[i] {
                    profit[i] = cur;
                }
                let t = profit[j] + cur;
                if t > maxprofit {
                    maxprofit = t;
                }
            }
        }
        maxprofit
    }
}
