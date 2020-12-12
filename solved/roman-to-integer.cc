class Solution {
public:
    int romanToInt(string s) {
        static int table[256] = {0};
        table['I'] = 1;
        table['V'] = 5;
        table['X'] = 10;
        table['L'] = 50;
        table['C'] = 100;
        table['D'] = 500;
        table['M'] = 1000;

        int sum = 0;
        for (size_t i = 0; i < s.length(); i++) {
            int n = table[s[i]];
            if (n == 0) {
                return 0;
            }
            int n2 = 0;
            if (i + 1 < s.length() && table[s[i+1]] > n) {
                n2 = n;
                n = table[s[i+1]];
                i++;
            }
            sum += n - n2;
        }
        return sum;
    }
};
