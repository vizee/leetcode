impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut m = nums.iter().enumerate().collect::<Vec<(usize, &i32)>>();
        m.sort_by(|a, b| a.1.cmp(&b.1));
        for (i, n) in nums.iter().enumerate() {
            match m.binary_search_by(|x| x.1.cmp(&(target - *n)).then(if x.0 == i { Ordering::Less } else { Ordering::Equal })) {
                Ok(p) => return vec![i as i32, m[p].0 as i32],
                _ => {}
            }
        }
        vec![]
    }
}
