impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        fn gen_next(p: &[u8]) -> Vec<usize> {
            let mut next = Vec::new();
            next.resize(p.len(), 0);
            let mut i = 1usize;
            let mut j = 0usize;
            while i < p.len() {
                while j > 0 && p[i] != p[j] {
                    j = next[j - 1];
                }
                if p[i] == p[j] {
                    j += 1;
                }
                next[i] = j;
                i += 1;
            }
            next
        }
        if needle.is_empty() {
            return 0;
        }
        let s = haystack.into_bytes();
        let p = needle.into_bytes();
        let next = gen_next(&p);
        let mut j = 0;
        for i in 0..s.len() {
            while j > 0 && s[i] != p[j] {
                j = next[j - 1];
            }
            if s[i] == p[j] {
                j += 1;
                if j == p.len() {
                    return (i + 1 - j) as i32;
                }
            }
        }
        -1
    }
}
