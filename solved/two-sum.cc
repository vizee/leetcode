class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        vector<int> m(nums);
        sort(m.begin(), m.end());
        vector<int> r;
        for (size_t i = 0; i < nums.size() && r.empty(); i++) {
            auto v = target - nums[i];
            auto p = lower_bound(m.begin(), m.end(), v);
            if (v != *p) {
                continue;
            }
            for (auto j = 0; j < nums.size(); j++) {
                if (nums[j] == v && j != i) {
                    r.push_back(i);
                    r.push_back(j);
                    break;
                }
            }
        }
        return r;
    }
};
