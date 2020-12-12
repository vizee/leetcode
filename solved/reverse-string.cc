class Solution {
public:
    string reverseString(string s)
    {
        string ss(s.length(), 0);
        auto i = s.length();
        for (auto c : s) {
            ss[--i] = c;
        }
        return ss;
    }
};
