class Solution {
public:
    int ladderLength(string beginWord, string endWord, unordered_set<string>& wordList) {
        auto q = queue<string>();
        q.push(beginWord);
        auto n = 1;
        auto d = 1;
        while (!q.empty()) {
            auto w = q.front();
            q.pop();
            for (size_t i = 0; i < w.length(); i++) {
                auto t = w[i];
                for (auto c = 'a'; c <= 'z'; c++) {
                    w[i] = c;
                    if (w == endWord) {
                        return d + 1;
                    }
                    auto it = wordList.find(w);
                    if (it != wordList.end()) {
                        q.push(*it);
                        wordList.erase(it);
                    }
                }
                w[i] = t;
            }
            n--;
            if (n == 0) {
                d++;
                n = q.size();
            }
        }
        return 0;
    }
};
