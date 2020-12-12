use std::collections::vec_deque::VecDeque;

const NIL: usize = 0;
const ROOT: usize = 1;

struct Node {
    trans: [usize; 128],
    fail: usize,
    end: usize,
}

impl Solution {
    fn new_node() -> Node {
        Node { trans: [0; 128], fail: NIL, end: 0 }
    }

    fn insert(dict: &mut Vec<Node>, s: String) {
        let mut p = ROOT;
        let n = s.len();
        for b in s.into_bytes() {
            let mut x = dict[p].trans[b as usize];
            if x == NIL {
                x = dict.len();
                dict.push(Self::new_node());
                dict[p].trans[b as usize] = x;
            }
            p = x;
        }
        dict[p].end = n;
    }

    fn build(dict: &mut Vec<Node>) {
        let mut q = VecDeque::with_capacity(26);
        for i in 0..128 {
            let p = dict[ROOT].trans[i];
            if p == NIL {
                continue;
            }
            dict[p].fail = ROOT;
            q.push_back(p);
        }
        while let Some(p) = q.pop_front() {
            for i in 0..128usize {
                let x = dict[p].trans[i];
                if x == NIL {
                    continue;
                }
                let mut fail = dict[p].fail;
                while fail != NIL {
                    let z = dict[fail].trans[i];
                    if z != NIL {
                        dict[x].fail = z;
                        break;
                    }
                    fail = dict[fail].fail;
                }
                if fail == NIL {
                    dict[x].fail = ROOT;
                }
                q.push_back(x);
            }
        }
    }

    fn query(nodes: Vec<Node>, s: String) -> Vec<String> {
        let mut end = Vec::new();
        end.resize(s.len(), false);
        let mut allpath = Vec::new();
        allpath.resize(s.len(), Vec::new());
        let mut p = ROOT;
        for (i, c) in s.as_bytes().iter().enumerate() {
            let c = *c as usize;
            while nodes[p].trans[c] == NIL && p != ROOT {
                p = nodes[p].fail;
            }
            p = nodes[p].trans[c];
            if p == NIL {
                p = ROOT;
            }
            let mut f = p;
            while f != ROOT {
                if nodes[f].end > 0 {
                    let n = nodes[f].end;
                    if i + 1 == n || end[i - n] {
                        end[i] = true;
                        allpath[i + 1 - n].push(n);
                    }
                }
                f = nodes[f].fail;
            }
        }
        if !*end.last().unwrap_or(&false) {
            return Vec::new();
        }

        fn dfs(s: &String, res: &mut Vec<String>, allpath: &mut Vec<Vec<usize>>, path: &mut Vec<usize>, p: usize) -> bool {
            let mut okpath = false;
            path.push(p);
            let n = allpath[p].len();
            for i in 0..n {
                if allpath[p][i] == 0 {
                    continue;
                }
                let e = p + allpath[p][i];
                if e == s.len() {
                    let mut buf = String::with_capacity(s.len() + path.len() - 1);
                    let mut b = 0;
                    for &e in path.iter().skip(1) {
                        buf.push_str(&s[b..e]);
                        buf.push(' ');
                        b = e;
                    }
                    buf.push_str(&s[b..]);
                    res.push(buf);
                }
                let ok = if e < s.len() { dfs(s, res, allpath, path, e) } else { e == s.len() };
                if !ok {
                    allpath[p][i] = 0;
                } else {
                    okpath = true;
                }
            }
            path.pop();
            if !okpath {
                allpath[p].clear();
            }
            okpath
        }

        let mut res = Vec::new();
        dfs(&s, &mut res, &mut allpath, &mut Vec::with_capacity(s.len()), 0);
        res
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut dict = Vec::with_capacity(word_dict.len() + 2);
        dict.push(Self::new_node());
        dict.push(Self::new_node());
        for w in word_dict {
            Self::insert(&mut dict, w);
        }
        Self::build(&mut dict);
        Self::query(dict, s)
    }
}
