impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        fn concat(words: &[String], max_width: usize, spaces: usize, last: bool) -> String {
            const SPACE: &str = " ";
            let mut s = String::with_capacity(max_width);
            if words.len() == 1 || last {
                s.push_str(&words.join(" "));
                s.push_str(&SPACE.repeat(spaces - words.len() + 1));
            } else {
                let slots = words.len() - 1;
                let white = spaces / slots;
                let mut remaining = spaces % slots;
                let mut spaces = spaces;
                for w in words {
                    if !s.is_empty() {
                        s.push_str(&SPACE.repeat(if remaining > 0 { remaining -= 1; white + 1} else { white }));
                    }
                    s.push_str(w);
                }
            }
            println!(">> {}", s);
            s
        }
        let max_width = max_width as usize;
        let mut r = Vec::new();
        let mut n = 0usize;
        let mut i = 0usize;
        let mut b = 0usize;
        loop {
            if i == words.len() || if n == 0 { 0 } else { n + 1 } + words[i].len() > max_width {
                let c = i - b;
                let spaces = max_width - n + c - 1;
                r.push(concat(&words[b..i], max_width, spaces, i == words.len()));
                if i == words.len() {
                    break;
                }
                b = i;
                n = 0;
            }
            if n > 0 {
                n += 1;
            }
            n += words[i].len();
            i += 1;
        }
        r
    }
}
