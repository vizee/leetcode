class Solution {
public:
    vector<int> findSubstring(string s, vector<string>& words) {
        auto n = words[0].length();
        if (s.length() < n * words.size()) {
            return vector<int>();
        }
        unordered_map<string, size_t> dict;
        for (size_t i = 0; i < words.size(); i++) {
            if (n != words[i].length()) {
                return vector<int>();
            }
            auto it = dict.find(words[i]);
            if (it == dict.end()) {
                dict.insert({words[i], words.size() + i});
            } else {
                it->second += words.size();
            }
        }
        auto l = n * words.size();
        vector<int> r;
        auto marks = vector<int>(words.size());
        int base = 0;
        for (size_t i = 0; i < s.length(); i++) {
            auto it = dict.find(s.substr(i, n));
            if (it == dict.end()) {
                continue;
            }
            size_t matched = 1;
            marks[it->second % words.size()] = base + 1;
            for (size_t j = i + n; j < i + l; j += n) {
                auto it = dict.find(s.substr(j, n));
                if (it == dict.end()) {
                    break;
                }
                auto idx = it->second % words.size();
                if (marks[idx] <= base) {
                    marks[idx] = base + 1;
                } else if (marks[idx] - base >= it->second / words.size()) {
                    break;
                } else {
                    marks[idx]++;
                }
                matched++;
            }
            if (matched == words.size()) {
                r.push_back(i);
            }
            base += words.size();
        }
        return r;
    }
};
