impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return Vec::new();
        }
        let mut nums = nums;
        nums.sort();
        let mut v = Vec::new();
        let mut i = 0;
        while i < nums.len() - 2 {
            let a = nums[i];
            let mut j = i + 1;
            while j < nums.len() - 1 {
                let b = nums[j];
                let c = 0 - a - b;
                if nums[j + 1..].binary_search(&c).is_ok() {
                    v.push(vec![a, b, c]);
                }
                while j < nums.len() - 1 && nums[j] == b {
                    j += 1;
                }
            }
            while i < nums.len() - 2 && nums[i] == a {
                i += 1;
            }
        }
        v
    }
}
