class Solution {
public:
    vector<int> intersection(vector<int>& nums1, vector<int>& nums2)
    {
        set<int> r;
        set<int> s;
        vector<int> v;
        for (auto n : nums1) {
            s.insert(n);
        }
        for (auto n : nums2) {
            if (s.find(n) != s.end() && r.find(n) == r.end()) {
                v.push_back(n);
                r.insert(n);
            }
        }
        sort(v.begin(), v.end());
        return v;
    }
};
