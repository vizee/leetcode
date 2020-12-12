impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        use std::mem::swap;
        /*
        1. local[i][j] = max(local[i][j - 1] + prices[j] - prices[j - 1], global[i - 1][j - 1] + max(prices[j] - prices[j - 1], 0))
        2. global[i][j] = max(global[i][j - 1], local[i][j])
        */
        let mut diff = prices;
        for i in (1..diff.len()).rev() {
            diff[i] = diff[i] - diff[i - 1];
        }
        let mut local = vec![0; diff.len()];
        let mut local0 = vec![0; diff.len()];
        let mut global = vec![0; diff.len()];
        let mut global0 = vec![0; diff.len()];
        for i in 0..k as usize {
            for j in 1..diff.len() {
                local[j] = (local[j - 1] + diff[j]).max(global0[j - 1] + diff[j].max(0));
                global[j] = global[j - 1].max(local[j]);
            }
            if *global0.last().unwrap_or(&0) == *global.last().unwrap_or(&0) {
                break;
            }
            swap(&mut local, &mut local0);
            swap(&mut global, &mut global0);
        }
        *global0.last().unwrap_or(&0)
    }
}
