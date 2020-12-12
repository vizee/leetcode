class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> vv;
        for (auto i = 0; i < numRows; i++) {
            vector<int> v(i + 1);
            for (auto j = 0; j < i + 1; j++) {
                if (j == 0 || j == i) {
                    v[j] = 1;
                } else {
                    v[j] = vv[i - 1][j - 1] + vv[i - 1][j];
                }
            }
            vv.push_back(v);
        }
        return vv;
    }
};
