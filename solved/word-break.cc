class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> dict(wordDict.begin(), wordDict.end());
        vector<int> t(s.length(), 0);
        auto scan = [&](int e) {
            for (int i = e - 1; i >= 0; i--) {
                if(t[i] != 0) {
                    continue;
                }
                auto ss = s.substr(i, e - i);
                if (dict.find(ss) != dict.end()) {
                    t[i] = 1;
                }
            }
        };
        int e = s.length();
        scan(e);
        while (t[0] != 1) {
            auto found = false;
            for (int i = 1; i < (int)t.size(); i++) {
                if(t[i] == 1) {
                    e = i;
                    t[i] = 2;
                    found = true;
                    break;
                }
            }
            if(!found) {
                return false;
            }
            scan(e);
        }
        return true;
    }
};
