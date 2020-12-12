class Solution {
public:
    bool isAnagram(string s, string t)
    {
        vector<int> v(26);
        for (auto c : s) {
            v[c - 'a']++;
        }
        for (auto c : t) {
            v[c - 'a']--;
        }
        for (auto n : v) {
            if (n) {
                return false;
            }
        }
        return true;
    }
};
