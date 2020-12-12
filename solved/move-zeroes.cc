class Solution {
public:
    void moveZeroes(vector<int>& nums)
    {
        auto p = 0;
        for (size_t i = 0; i < nums.size(); i++) {
            if (nums[i]) {
                nums[p++] = nums[i];
            }
        }
        for (size_t i = p; i < nums.size(); i++) {
            nums[i] = 0;
        }
        return;
    }
};
