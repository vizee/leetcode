class Solution {
public:
    int lengthOfLastWord(string s) {
        int b = -1;
        for (int i = (int)s.length() - 1; i >= 0; i--) {
            if (s[i] != ' ') {
                if (b == -1) {
                    b = i;
                }
            } else {
                if (b > 0) {
                    return b - i;
                }
            }
        }
        return b + 1;
    }
};
