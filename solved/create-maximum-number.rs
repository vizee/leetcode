impl Solution {
    fn merge(res: &mut [i32], a: &[i32], b: &[i32]) {
        let mut p = 0usize;
        let mut i = 0usize;
        let mut j = 0usize;
        while i < a.len() && j < b.len() {
            if &a[i..] > &b[j..] {
                res[p] = a[i];
                p += 1;
                i += 1;
            } else {
                res[p] = b[j];
                p += 1;
                j += 1;
            }
        }
        res[p..].copy_from_slice(if i < a.len() { &a[i..] } else { &b[j..] });
    }

    fn max_nth(a: &[i32], m: usize) -> usize {
        let mut i = 0usize;
        for j in 0..a.len() {
            if j >= m || a[i] == 9 {
                break;
            }
            if a[j] > a[i] {
                i = j;
            }
        }
        i
    }

    fn k_nums(res: &mut [i32], a: &[i32]) -> bool {
        if res.is_empty() {
            return true;
        }
        if a.len() < res.len() {
            return false;
        }

        // TODO: use mono-stack

        let mut p = 0usize;
        let mut i = 0usize;
        while p < res.len() && a.len() - i > res.len() - p {
            let k = res.len() - p;
            let x = Self::max_nth(&a[i..], a.len() - i - k + 1);
            res[p] = a[i + x];
            p += 1;
            i += x + 1;
        }

        if a.len() - i == res.len() - p {
            res[p..].copy_from_slice(&a[i..]);
        }

        true
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut res = vec![0i32; k];

        let mut s1 = vec![0i32; k];
        let mut s2 = vec![0i32; k];
        let mut tmp = vec![0i32; k];

        for l in 0..=k {
            if Self::k_nums(&mut s1[..l], &nums1) && Self::k_nums(&mut s2[..k - l], &nums2) {
                Self::merge(&mut tmp, &s1[..l], &s2[..k - l]);
                if tmp > res {
                    res.copy_from_slice(&tmp);
                }
            }
        }

        res
    }
}
