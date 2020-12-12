impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || t.is_empty() || t.len() > s.len() {
            return "".to_string();
        }
        let mut table = [0usize; 256];
        let t = t.into_bytes();
        for c in t.iter() {
            table[*c as usize] += 1;
        }
        let mut count = 0usize;
        let mut window = [0usize; 256];
        let mut right = 0usize;
        let s = s.into_bytes();
        for (i, c) in s.iter().enumerate() {
            let c = *c as usize;
            if table[c] == 0 {
                continue;
            }
            window[c] += 1;
            if window[c] <= table[c] {
                count += 1;
                if count == t.len() {
                    right = i;
                    break;
                }
            }
        }
        if count != t.len() {
            return "".to_string();
        }
        let mut left = 0usize;
        let mut n = right + 1;
        for i in 0..s.len() - t.len() {
            let c1 = s[i] as usize;
            if window[c1] > 0 {
                window[c1] -= 1;
            }
            if window[c1] < table[c1] {
                let mut r = None;
                for j in right + 1..s.len() {
                    let c2 = s[j] as usize;
                    if table[c2] == 0 {
                        continue;
                    }
                    window[c2] += 1;
                    if c2 == c1 {
                        r = Some(j);
                        break;
                    }
                }
                if let Some(r) = r {
                    right = r;
                } else {
                    break;
                }
            }
            let m = right - i;
            if m < n {
                left = i + 1;
                n = m;
            }
        }
        String::from_utf8(s[left..left+n].to_vec()).unwrap()
    }
}
