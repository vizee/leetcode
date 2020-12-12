impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::with_capacity(nums.len());
        let mut max = 0;
        for v in nums {
            if count.contains_key(&v) {
                continue;
            }
            let prev = *count.get(&(v - 1)).unwrap_or(&0);
            let next = *count.get(&(v + 1)).unwrap_or(&0);
            let n = 1 + prev + next;
            count.insert(v, n);
            if prev != 0 {
                count.entry(v - prev).and_modify(|v| *v = n);
            }
            if next != 0 {
                count.entry(v + next).and_modify(|v| *v = n);
            }
            max = max.max(n);
        }
        max
    }
}
