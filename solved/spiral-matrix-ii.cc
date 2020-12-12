class Solution {
public:
    vector<vector<int>> generateMatrix(int n) {
        auto vji = vector<vector<int>>(n);
        for (auto i = 0; i < n; i++) {
            vji[i] = vector<int>(n, 0);
        }
        if (n == 0) {
            return vji;
        }
        if (n == 1) {
            vji[0][0] = 1;
            return vji;
        }
        auto turn = false;
        auto c = 1;
        auto di = 1, dj = 0;
        auto i = 0, j = 0;
        while (vji[j][i] == 0) {
            vji[j][i] = c++;
            if (i + di >= n || j + dj >= n || i + di < 0) {
                turn = true;
            } else if (vji[j + dj][i + di] != 0) {
                turn = true;
            }
            if (turn) {
                if (di == 1) {
                    di = 0;
                    dj = 1;
                } else if (dj == 1) {
                    di = -1;
                    dj = 0;
                } else if (di == -1) {
                    di = 0;
                    dj = -1;
                } else if (dj == -1) {
                    di = 1;
                    dj = 0;
                }
                turn = false;
            }
            i += di;
            j += dj;
        }
        return vji;
    }
};
