class Solution {
public:
    int minSubArrayLen(int s, vector<int>& nums) {
        auto b = 0;
        auto e = 0;
        auto sum = 0;
        auto r = 0;
        while(e < nums.size()) {
            sum += nums[e];
            while(sum >= s) {
                   if (e - b + 1 < r || r == 0) {
                       r = e - b + 1;
                   }
                sum -= nums[b++];
            }
            e++;
        }
        return r;
    }
};
