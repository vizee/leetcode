class Solution {
public:
    int maxArea(vector<int>& height) {
        if (height.size() < 2) {
            return 0;
        }
        size_t a = 0;
        size_t b = height.size() - 1;
        int m = -1;
        while (a < b) {
            int x = (b - a) * min(height[a], height[b]);
            if (x > m) {
                m = x;
            }
            if (height[a] < height[b]) {
                a++;
            } else {
                b--;
            }
        }
        return m;
    }
};
