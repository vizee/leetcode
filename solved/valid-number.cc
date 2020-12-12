class Solution {
public:
    bool isNumber(string s) {
        bool num = false;
        bool frac = false;
        int exp = 0;
        bool final = false;
        auto p = s.c_str();
space_begin:
        // \s*
        while (isspace(*p)) {
            p++;
        }
        if (final) {
            goto num_ret;
        }
exp_begin:
        // [\+-]?
        if (*p == '+' || *p == '-') {
            p++;
        }
        // \d+
frac_begin:
        while (*p >= '0' && *p <= '9') {
            p++;
            num = true;
            if (exp == 1) {
                exp = 2;
            }
        }
        if (exp != 0) {
            goto exp_end;
        }
        // (.\d+)?
        if (!frac && *p == '.') {
            p++;
            frac = true;
            goto frac_begin;
        }
        // (e\d+)?
        if (num && exp == 0 && (*p == 'e' || *p == 'E')) {
            p++;
            exp = 1;
            goto exp_begin;
        }
exp_end:
        final = true;
        goto space_begin;
num_ret:
        return num && exp != 1 && *p == '\0';
    }


};
