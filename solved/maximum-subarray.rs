impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter().fold((*nums.first().unwrap_or(&0), 0), |(max, cur), &x| {
            let next = cur + x;
            (max.max(next), if next > 0 { next } else { 0 })
        }).0
    }
}
