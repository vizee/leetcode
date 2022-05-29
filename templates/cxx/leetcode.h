#ifndef _LEETCODE_H_
#define _LEETCODE_H_

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef __cplusplus
#include <algorithm>
#include <string>
#include <vector>
#include <map>
#include <unordered_map>
#include <unordered_set>
#include <list>
#include <deque>
#include <queue>
#include <regex>
#include <tuple>
#include <iostream>

using namespace std;

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

void println() {
    cout << endl;
}

template<typename T>
void println(T v) {
    cout << v << endl;
}

template<typename T>
void println(vector<T>& v) {
    for (auto &i: v) {
        cout << i << " ";
    }
    cout << endl;
}

template<typename T, typename... Args>
void println(T t, Args... args) {
    cout << t << " ";
    println(args...);
}

#else

struct ListNode {
    int val;
    struct ListNode *next;
};

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

#endif

#endif
