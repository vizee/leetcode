class Solution {
public:
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> vv;
        vector<int> v;
        for (int i = 1; i <= k; i++) {
            v.push_back(i);
        }
        int p = k - 1;
        for (;;) {
            if (p == k - 1) {
                vector<int> t = v;
                vv.push_back(t);
            }
            v[p]++;
            if (v[p] > n - (k - p - 1)) {
                if (p == 0) {
                    break;
                }
                p--;
            } else if (p < k - 1) {
                for (int i = p + 1; i < k; i++) {
                    v[i] = v[i - 1] + 1;
                }
                p = k - 1;
            }
        }
        return vv;
    }
};
