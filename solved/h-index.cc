class Solution {
public:
    int hIndex(vector<int>& citations) {
        sort(citations.rbegin(), citations.rend());
        for(auto i = 0; i <= (int)citations.size(); i++) {
            if (i == citations.size() || citations[i] < i + 1) {
                return i;
            }
        }
        return 0;
    }
};
