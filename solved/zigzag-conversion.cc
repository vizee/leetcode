class Solution {
public:
    string convert(string s, int numRows) {
        string d(s.length(), ' ');
        int step = numRows > 2 ? (numRows - 1) * 2 : numRows;
        int k = 0;
        for (int i = 0; i < numRows; i++) {
            int n = step - i * 2;
            for (int j = i; j < s.length(); j += step) {
                d[k++] = s[j];
                if (i > 0 && i < numRows - 1 && j + n < s.length()) {
                    d[k++] = s[j + n];
                }
            }
        }
        return d;
    }
};
