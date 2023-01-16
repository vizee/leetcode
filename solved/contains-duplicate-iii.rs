impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        let mut s = std::collections::BTreeMap::<i32, i32>::new();
        for i in 0..nums.len() {
            if s.range(nums[i] - value_diff..=nums[i] + value_diff).next().is_some() {
                return true;
            }
            *s.entry(nums[i]).or_default() += 1;
            if s.len() > index_diff as usize {
                let v = s.get_mut(&nums[i - index_diff as usize]).unwrap();
                *v -= 1;
                if *v == 0 {
                    s.remove(&nums[i - index_diff as usize]);
                }
            }
        }
        false
    }
}
