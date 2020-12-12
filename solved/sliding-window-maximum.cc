class Solution {
public:
    inline int max_index(vector<int>& a, int b, int e) {
        int m = b;
        if (e > (int)a.size()) {
            e = (int)a.size();
        }
        while (b < e) {
            if (a[b] > a[m]) {
                m = b;
            }
            b++;
        }
        return m;
    }

    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        vector<int> v;
        if (k == 0) {
            return v;
        }
        auto m = -1;
        for (auto i = k - 1; i < (int)nums.size(); i++) {
            if(i - k >= m) {
                m = max_index(nums, m + 1, m + 1 + k);
            } else if(nums[i] >= nums[m]) {
                m = i;
            }
            v.push_back(nums[m]);
        }
        return v;
    }
};
