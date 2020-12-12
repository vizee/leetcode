impl Solution {
    fn is_palindrome(s: &[u8]) -> bool {
        let mut i1;
        let mut i2;
        if s.len() % 2 == 0 {
            i2 = s.len() / 2;
            i1 = i2 - 1;
        } else {
            i1 = s.len() / 2;
            i2 = i1;
        }
        while i2 < s.len() && s[i1] == s[i2] {
            if i1 == 0 {
                return true;
            }
            i1 -= 1;
            i2 += 1;
        }
        false
    }

    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let t = s.as_bytes();
        let mut p = t.len();
        loop {
            p = t[..p].iter().rposition(|&c| c == t[0]).unwrap();
            if Self::is_palindrome(&t[..=p]) {
                break;
            }
        }
        if p + 1 < t.len() {
            let m = t.len() - p - 1;
            let mut res = String::with_capacity(t.len() + m);
            res.push_str(&s[s.len() - m..].chars().rev().collect::<String>());
            res.push_str(&s);
            res
        } else {
            s
        }
    }
}
