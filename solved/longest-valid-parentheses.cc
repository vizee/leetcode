class Solution {
public:
    int longestValidParentheses(string s) {
        int n = 0;
        int lparan = 0;
        auto matched = vector<int>(s.length());
        for (size_t i = 0; i < s.length(); i++) {
            if (s[i] == '(') {
                lparan++;
                continue;
            }
            if (lparan == 0) {
                matched[i] = -1;
                continue;
            }
            lparan--;
            int lpos = i - 1;
            while (lpos > 0) {
                if (s[lpos] == ')') {
                    lpos = matched[lpos] - 1;
                } else if (s[lpos - 1] == ')' && matched[lpos - 1] != -1) {
                    lpos = matched[lpos - 1];
                    break;
                } else {
                    break;
                }
            }
            matched[i] = lpos;
            if (i - lpos + 1 > n) {
                n = i - lpos + 1;
            }
        }
        return n;
    }
};
