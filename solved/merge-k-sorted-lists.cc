class Solution {
public:
    vector<ListNode*> h;

    void siftup(int i) {
        auto v = h[i];
        while (i > 0) {
            auto p = (i - 1) / 2;
            if (h[p]->val <= v->val) {
                break;
            }
            h[i] = h[p];
            i = p;
        }
        h[i] = v;
    }

    void siftdown(int i) {
        auto v = h[i];
        int n = h.size() / 2;
        while (i < n) {
            auto min = 2 * i + 1;
            if (min + 1 < (int)h.size() && h[min + 1]->val < h[min]->val) {
                min++;
            }
            if (h[min]->val >= v->val) {
                break;
            }
            h[i] = h[min];a
            i = min;
        }
        h[i] = v;
    }

    ListNode* mergeKLists(vector<ListNode*>& lists) {
        int i = 0;
        int j = (int)lists.size() - 1;
        while (i <= j) {
            if (lists[i] != nullptr) {
                i++;
            } else {
                lists[i] = lists[j--];
                lists.pop_back();
            }
        }
        if (lists.empty()) {
            return nullptr;
        }
        h.clear();
        for (auto v : lists) {
            h.push_back(v);
            siftup(h.size() - 1);
        }
        ListNode list(0);
        ListNode *node = &list;
        while (!h.empty()) {
            siftdown(0);
            node->next = h[0];
            node = h[0];
            h[0] = h[0]->next;
            if (h[0] == nullptr) {
                h[0] = h.back();
                h.pop_back();
            }
        }
        return list.next;
    }
};
