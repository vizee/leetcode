impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let begin_word = begin_word.into_bytes();
        let end_word = end_word.into_bytes();
        let mut dict = HashSet::with_capacity(word_list.len());
        word_list.iter().for_each(|w| {
            dict.insert(w.clone().into_bytes());
        });
        let mut q = std::collections::VecDeque::new();
        q.push_back(begin_word);
        let mut d = 1;
        let mut fan = 1usize;
        while let Some(mut w) = q.pop_front() {
            for i in 0..w.len() {
                let t = w[i];
                for c in b'a'..=b'z' {
                    if c == t {
                        continue;
                    }
                    w[i] = c;
                    if dict.remove(&w) {
                        if w == end_word {
                            return d + 1;
                        }
                        q.push_back(w.clone());
                    }
                }
                w[i] = t;
            }
            fan -= 1;
            if fan == 0 {
                d += 1;
                fan = q.len();
            }
        }
        0
    }
}
