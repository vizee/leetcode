const NIL: usize = 0;
const ROOT: usize = 1;

struct Node {
    next: [usize; 26],
    end: usize,
    used: usize,
}

impl Node {
    fn new() -> Node {
        Node { next: [NIL; 26], end: 0, used: 0 }
    }
}

struct Trie {
    all: Vec<Node>,
}

impl Trie {
    fn new() -> Trie {
        let mut v = Vec::new();
        v.push(Node::new());
        v.push(Node::new());
        Trie { all: v }
    }

    fn add(&mut self, s: &[u8], id: usize) {
        let mut p = ROOT;
        for &c in s {
            let next = self.all[p].next[(c - b'a') as usize];
            if next == NIL {
                let next = self.all.len();
                self.all.push(Node::new());
                self.all[p].next[(c - b'a') as usize] = next;
                p = next;
            } else {
                p = next;
            }
        }
        self.all[p].end = id;
    }

    fn query(&self, start: usize, word: &[u8]) -> Option<usize> {
        let mut p = start;
        for &c in word {
            p = self.all[p].next[(c - b'a') as usize];
            if p == NIL {
                return None;
            }
        }
        Some(p)
    }

    fn match_all(&mut self, deep: usize, word: &[u8]) -> Vec<usize> {
        let mut all = Vec::new();
        for i in 0..word.len() {
            if let Some(p) = self.query(ROOT, &word[..i]) {
                let me = (word[i] - b'a') as usize;
                for j in 0..26 {
                    let next = self.all[p].next[j];
                    if me == j || next == NIL {
                        continue;
                    }
                    if let Some(end) = self.query(next, &word[i + 1..]) {
                        if self.all[end].used == 0 || self.all[end].used == deep {
                            all.push(end);
                            self.all[end].used = deep;
                        }
                    }
                }
            } else {
                break;
            }
        }
        all
    }
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::vec_deque::VecDeque;

        let mut dict = Trie::new();
        for (i, s) in word_list.iter().enumerate() {
            dict.add(s.as_bytes(), i);
        }
        let target = if let Some(p) = dict.query(ROOT, end_word.as_bytes()) {
            p
        } else {
            return Vec::new();
        };

        let mut res = Vec::new();
        let mut max_deep = word_list.len() - 1;
        let mut q = VecDeque::<(Vec<usize>, Vec<usize>)>::new();
        q.push_back((Vec::new(), dict.match_all(0, begin_word.as_bytes())));
        while let Some((path, next)) = q.pop_front() {
            if path.len() > max_deep {
                continue;
            }
            if next.contains(&target) {
                let mut r = vec![begin_word.clone()];
                path.iter().map(|&p| word_list[dict.all[p].end].clone()).for_each(|w| r.push(w));
                r.push(end_word.clone());
                res.push(r);
                max_deep = path.len();
                continue;
            }
            if path.len() >= max_deep {
                continue;
            }
            for x in next {
                let new_next = dict.match_all(path.len() + 1, word_list[dict.all[x].end].as_bytes());
                if new_next.is_empty() {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(x);
                q.push_back((new_path, new_next));
            }
        }
        res
    }
}
