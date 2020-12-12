int lengthOfLongestSubstring(char* s)
{
    int v[128];
    int i;
    int r = 0;
    int p = 0;
    int b = 0;
    int t;

    for (i = 0; i < 128; i++) {
        v[i] = -1;
    }
    if (!*s) {
        return 0;
    }
    while (*s) {
        t = v[*s];
        if (t >= b) {
            if (p - b > r) {
                r = p - b;
            }
            b = t + 1;
        }
        v[*s] = p;
        p++;
        s++;
    }
    return p - b > r ? p - b : r;
}
