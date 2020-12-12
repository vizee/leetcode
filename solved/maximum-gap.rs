impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let (min, max) = nums
            .iter()
            .map(|&x| x as i64)
            .fold((nums[0] as i64, nums[0] as i64), |(min, max), x| (min.min(x), max.max(x)));

        let n = nums.len() as i64;
        let m = max - min;
        if m == 0 {
            return 0;
        }

        let mut buckets = Vec::new();
        buckets.resize(nums.len() + 1, (max, min));
        nums.iter()
            .map(|&x| x as i64)
            .for_each(|x| {
                let b = &mut buckets[((x - min) * n / m) as usize];
                *b = (x.min(b.0), x.max(b.1));
            });
        let mut res = 0i64;
        let mut prev = 0usize;
        for i in 1..buckets.len() {
            if buckets[i].0 > buckets[i].1 {
                continue;
            }
            res = res.max(buckets[i].0 - buckets[prev].1);
            prev = i;
        }
        res as i32
    }
}
