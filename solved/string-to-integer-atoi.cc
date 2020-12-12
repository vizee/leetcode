class Solution {
public:
    int myAtoi(string str) {
        int n = 0;
        int neg = 0;
        auto p = str.c_str();
        while (*p != 0) {
            if (isdigit(*p)) {
                if ((n == -214748364 && *p == '9') || n < -214748364) {
                    n = INT_MIN;
                    break;
                }
                n = n * 10 - (*p - '0');
            } else if (*p == '-' || *p == '+') {
                if (n != 0 || neg != 0) {
                    break;
                }
                neg = *p == '-' ? 1 : 2;
            } else if (isspace(*p)) {
                if ((neg != 0 || n != 0)) {
                    break;
                }
            } else {
                break;
            }
            p++;
        }
        return neg == 1 ? n : n == INT_MIN ? INT_MAX : -n;
    }
};
