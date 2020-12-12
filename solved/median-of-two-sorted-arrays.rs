impl Solution {
    fn find_kth(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        match (nums1.len(), nums2.len()) {
            (0, 0) => 0,
            (0, _) => nums2[k],
            (_, 0) => nums1[k],
            (n, m) => if k == 0 {
                nums1[0].min(nums2[0])
            } else {
                let half_k = (k - 1) / 2;
                let i = if half_k >= n { n - 1 } else { half_k };
                let j = if half_k >= m { m - 1 } else { half_k };
                if nums1[i] < nums2[j] {
                    Self::find_kth(&nums1[i + 1..], nums2, k - i - 1)
                } else {
                    Self::find_kth(nums1, &nums2[j + 1..], k - j - 1)
                }
            }
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let k = (nums1.len() + nums2.len()) / 2;
        let v = Self::find_kth(&nums1, &nums2, k);
        if (nums1.len() + nums2.len()) % 2 == 0 && k > 0 {
            let v1 = Self::find_kth(&nums1, &nums2, k - 1);
            (v + v1) as f64 / 2f64
        } else {
            v as f64
        }
    }
}
