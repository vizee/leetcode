class Solution {
public:
    ListNode* merge(ListNode *a, ListNode *b) {
        auto l = a->val < b->val ? a : b;
        while (a != nullptr) {
            if (a->val >= b->val) {
                swap(a, b);
            }
            auto v = b->val;
            // a->val < b->val
            while (a->next != nullptr && a->next->val < b->val) {
                a = a->next;
            }
            auto next = a->next;
            a->next = b;
            a = next;
        }
        return l;
    }

    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        if (l1 == nullptr) {
            return l2;
        }
        if (l2 == nullptr) {
            return l1;
        }
        return merge(l1, l2);
    }
};
