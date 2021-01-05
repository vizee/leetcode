impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return Vec::new();
        }
        nums.sort();
        let mut res = Vec::new();
        let mut a = nums[0] - 1;
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            if nums[i] == a {
                continue;
            }
            a = nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            let mut last = -1;
            while j < k {
                if a + nums[j] > 0 {
                    break;
                }
                let x = a + nums[j] + nums[k];
                if last != 0 && x == 0 {
                    res.push(vec![a, nums[j], nums[k]]);
                }
                last = x;
                if x > 0 {
                    k -= 1
                } else {
                    j += 1;
                }
            }
        }
        res
    }
}
