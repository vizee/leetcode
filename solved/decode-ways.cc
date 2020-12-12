class Solution {
public:
    int numDecodings(string s) {
        /*
        10 1 10 20 20 1 20 2 10
        */
        if (s.length() == 0) {
            return 0;
        }
        if (s[0] == '0') {
            return 0;
        }
        if (s.length() == 1) {
            return 1;
        }
        if (s[0] == '1' || (s[0] == '2' && s[1] <= '6')) {
            if (s.length() == 2) {
                return s[1] == '0' ? 1 : 2;
            }
            if (s[1] == '0') {
                return numDecodings(s.substr(2));
            }
            auto r = 0;
            if (s[1] == '1' || (s[1] == '2' && s[2] <= '6')) {
                //1 10 1 10
                if (s.length() == 3) {
                    r = 1;
                } else {
                    r = numDecodings(s.substr(3));
                }
                if (s[2] == '0') {
                    return r;
                }
            }
            return 2 * numDecodings(s.substr(2)) + r;
        }
        return numDecodings(s.substr(1));
    }
};
