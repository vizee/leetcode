class Solution {
public:
    template<char I, char V, char X>
    void digitToRoman(string &s, int num) {
        switch (num) {
        case 0:
            break;
        case 9:
            s.push_back(I);
            s.push_back(X);
            break;
        case 4:
            s.push_back(I);
            s.push_back(V);
            break;
        default:
            if (num >= 5) {
                s.push_back(V);
                num -= 5;
            }
            for (int i = 0; i < num; i++) {
                s.push_back(I);
            }
        }
    }

    string intToRoman(int num) {
        string s;
        s.reserve(16);
        digitToRoman<'M', ' ', ' '>(s, num / 1000);
        num %= 1000;
        digitToRoman<'C', 'D', 'M'>(s, num / 100);
        num %= 100;
        digitToRoman<'X', 'L', 'C'>(s, num / 10);
        num %= 10;
        digitToRoman<'I', 'V', 'X'>(s, num);
        return s;
    }
};
