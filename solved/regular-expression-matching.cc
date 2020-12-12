class Solution {
public:
    struct nfa_state {
        char c;
        bool any;
        bool accepted;
        vector<int> next;

        nfa_state(char c) : next() {
            accepted = false;
            any = false;
            this->c = c;
        }
    };

    vector<nfa_state> build_nfa(const char *s) {
        vector<nfa_state> states;
        states.push_back(nfa_state(0));
        vector<int> prevs = {0};
        for (size_t i = 0; s[i]; i++) {
            states.push_back(nfa_state(s[i]));
            auto id = (int)states.size() - 1;
            auto& st = states[id];
            if (s[i + 1] == '*') {
                st.any = true;
                i++;
            }
            for (auto pi : prevs) {
                states[pi].next.push_back(id);
            }
            if (st.any) {
                st.next.push_back(id);
            } else {
                prevs.clear();
            }
            prevs.push_back(id);
        }
        for (auto pi : prevs) {
            states[pi].accepted = true;
        }
        return states;
    }

    bool nfa_match(vector<nfa_state> &states, string &s) {
        unordered_set<int> next;
        unordered_set<int> cur;
        auto accepted = states[0].accepted;
        for (auto i : states[0].next) {
            cur.insert(i);
            if (!accepted) {
                accepted = states[i].any && states[i].accepted;
            }
        }
        auto p = 0;
        while (!cur.empty() && p != s.length()) {
            next.clear();
            accepted = false;
            for (auto it = cur.begin(); it != cur.end(); it++) {
                auto &st = states[*it];
                if (st.c == '.' || st.c == s[p]) {
                    if (!accepted) {
                        accepted = st.accepted;
                    }
                    for (auto i : st.next) {
                        next.insert(i);
                    }
                }
            }
            p++;
            swap(cur, next);
        }
        return accepted && p == s.length();
    }

    bool isMatch(string s, string p) {
        auto states = build_nfa(p.c_str());
        return nfa_match(states, s);
    }

};

