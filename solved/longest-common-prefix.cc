class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) {
            return "";
        }
        if (strs.size() == 1) {
            return strs[0];
        }
        auto& s0 = strs[0];
        for (size_t i = 0; i < s0.length(); i++) {
            char c = s0[i];
            bool nomatch = false;
            for (auto &s: strs) {
                if (c != s[i]) {
                    nomatch = true;
                    break;
                }
            }
            if (nomatch) {
                return s0.substr(0, i);
            }
        }
        return s0;
    }
};
