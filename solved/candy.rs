impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candy = Vec::with_capacity(ratings.len());
        candy.resize(ratings.len(), 1);
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                candy[i] = candy[i - 1] + 1;
            }
        }
        let mut rcandy = Vec::with_capacity(ratings.len());
        rcandy.resize(ratings.len(), 1);
        let mut i = ratings.len() - 1;
        while i > 0 {
            let j = i - 1;
            if ratings[j] > ratings[i] {
                rcandy[j] = rcandy[i] + 1;
            }
            i = j;
        }
        for i in 0..ratings.len() {
            candy[i] = candy[i].max(rcandy[i]);
        }
        candy.iter().sum()
    }
}
