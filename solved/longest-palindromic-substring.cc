class Solution {
public:
    string longestPalindrome(string s) {
        auto l = (int)s.length();
        auto f = [&s,l](int b, int e) -> decltype(e - b) {
            if (s[b] != s[e]) {
                return -1;
            }
            while (b - 1 >= 0 && e + 1 < l) {
                if (s[b - 1] != s[e + 1]) {
                    break;
                }
                b--;
                e++;
            }
            return (e - b) / 2;
        };
        auto xi = 0;
        auto xn = 0;
        auto xd = false;
        for (int i = 0; i < l; i++) {
            if (i + xn >= l) {
                break;
            }
            int il = i - xn, ir = i + xn + 1;
            if (!xd) {
                il += 1;
                ir -= 1;
            }
            auto b1 = s[i - xn] != s[i + xn];
            auto b2 = (ir > l || s[il] != s[ir]);
            if (b1 && b2) {
                continue;
            }
            auto n1 = b1 ? 0 : f(i, i);
            auto n2 = b2 ? -1 : f(i, i + 1);
            auto nx = max(n1, n2);
            if (nx >= xn || (n2 >= n1 && !xd && n2 == xn - 1)) {
                xn = nx + 1;
                xi = i;
                xd = n2 >= n1;
            }
        }
        int n = xn - 1;
        return s.substr(xi - n, n * 2 + (xd ? 2 : 1));
    }
};
