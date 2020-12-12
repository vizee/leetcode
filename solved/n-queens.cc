class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        vector<vector<string>> vv;
        vector<bool> cols(n, false);
        vector<bool> d(n + n, false);
        vector<bool> bd(n + n, false);
        vector<int> stack;
        int nextx = 0;
        do {
            auto y = stack.size();
            auto pop = true;
            for (auto x = nextx; x < n; x++) {
                if(cols[x] || d[n - 1 + x - y] || bd[x + y]) {
                    continue;
                }
                if (y == n - 1) {
                    vector<string> v;
                    stack.push_back(x);
                    for (auto i : stack) {
                        string s(n, '.');
                        s[i] = 'Q';
                        v.push_back(s);
                    }
                    stack.pop_back();
                    vv.push_back(v);
                } else {
                    stack.push_back(x);
                    //cout << "push: " << x << ", " << y << endl;
                    cols[x] = true;
                    d[n - 1 + x - y] = true;
                    bd[x + y] = true;
                    pop = false;
                    break;
                }
            }
            if (pop && !stack.empty()) {
                auto x = stack.back();
                stack.pop_back();
                auto y = stack.size();
                //cout << "pop: " << x << ", " << y << endl;
                cols[x] = false;
                d[n - 1 + x - y] = false;
                bd[x + y] = false;
                nextx = x + 1;
            } else {
                nextx = 0;
            }
        } while (!stack.empty() || nextx > 0);
        return vv;
    }
};
