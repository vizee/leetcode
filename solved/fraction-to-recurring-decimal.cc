class Solution {
public:
    string fractionToDecimal(int numerator, int denominator) {
        if (numerator == 0) {
            return "0";
        }
        map<long long, int> f;
        vector<long long> v;
        auto neg = false;
        auto c = 0;
        auto d = -1;
        auto r = (long long)numerator;
        auto m = (long long)denominator;
        if (r < 0) {
            neg = !neg;
            r = -r;
        }
        if (m < 0) {
            neg = !neg;
            m = -m;
        }
        while (r != 0) {
            v.push_back(r / m);
            c++;
            r = r % m;
            auto it = f.find(r);
            if (it == f.end()) {
                f[r] = c;
            } else {
                d = f[r];
                break;
            }
            r *= 10;
        }
        ostringstream ss;
        if (neg) {
            ss << "-";
        }
        for (size_t i = 0; i < v.size(); i++) {
            if (i == 1) {
                ss << ".";
            }
            if (d == i) {
                ss << "(";
            }
            ss << v[i];
        }
        if (d != -1) {
            ss << ")";
        }
        return ss.str();
    }
};
