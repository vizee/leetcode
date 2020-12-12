class Solution {
public:
    int addDigits(int num) {
        int n;

        while (num >= 10) {
            n = 0;
            while (num) {
                n += num % 10;
                num /= 10;
            }
            num = n;
        }
        return num;
    }

};
