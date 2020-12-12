class Solution {
public:
    bool exist(vector<vector<char>>& board, string word) {
        auto dfs = [&](int x, int y) -> bool {
            vector<int> v;
            auto p = 0;
            //cout << ">>" << x << "," << y << "-" << board[x][y] << endl;
            board[x][y] = '\x0';
            p++;
            auto nextd = 0;
            while(p < (int)word.length()) {
                auto pop = true;
                for (int d = nextd; d < 4; d++) {
                    auto dx = 0;
                    auto dy = 0;
                    switch (d) {
                    case 0:
                        dx = 1;
                        break;
                    case 1:
                        dy = 1;
                        break;
                    case 2:
                        dx = -1;
                        break;
                    case 3:
                        dy = -1;
                        break;
                    }
                    if (x + dx < 0
                        || x + dx >= (int)board.size()
                        || y + dy < 0
                        || y + dy >= (int)board[x].size()) {
                        continue;
                    }
                    if(word[p] == board[x + dx][y + dy]) {
                        v.push_back(d);
                        x += dx;
                        y += dy;
                        //cout << x << "," << y << "-" << board[x][y] << endl;
                        board[x][y] = '\x0';
                        p++;
                        pop = false;
                        break;
                    }
                }
                if (pop) {
                    if (v.empty()) {
                        break;
                    }
                    p--;
                    board[x][y] = word[p];
                    //cout << x << "," << y << "+" << word[p] << endl;
                    nextd = v.back();
                    v.pop_back();
                    auto dx = 0;
                    auto dy = 0;
                    switch (nextd) {
                    case 0:
                        dx = 1;
                        break;
                    case 1:
                        dy = 1;
                        break;
                    case 2:
                        dx = -1;
                        break;
                    case 3:
                        dy = -1;
                        break;
                    }
                    x -= dx;
                    y -= dy;
                    nextd++;
                } else {
                    nextd = 0;
                }
            }
            //cout << "<<" << x << "," << y << "+" << word[0] << endl;
            board[x][y] = word[0];
            return p == (int)word.length();
        };
        for (int i = 0; i < (int)board.size(); i++) {
            for (int j = 0; j < (int)board[i].size(); j++) {
                if(board[i][j] != word[0]) {
                    continue;
                }
                if (dfs(i, j)) {
                    return true;
                }
            }
        }
        return false;
    }
};
