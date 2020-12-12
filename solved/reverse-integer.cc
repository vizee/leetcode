class Solution {
public:
    int reverse(int x) {
        long long n = x;
        bool neg = x < 0;
        if (neg) {
            n = -n;
        }
        long long v = 0;
        while (n > 0) {
            v = v * 10 + n % 10;
            n /= 10;
        }
        if ((!neg && v > 0x7fffffff) || (neg && v > 0x80000000)) {
            return 0;
        }
        return (int)(neg ? -v : v);
    }
};
