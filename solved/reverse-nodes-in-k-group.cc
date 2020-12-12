class Solution {
public:
    ListNode* reverseKGroup(ListNode* head, int k) {
        if (head == nullptr || k == 1) {
            return head;
        }
        auto n = head;
        ListNode *g = nullptr;
        while (n != nullptr) {
            ListNode *t = nullptr;
            ListNode *h = nullptr;
            int i;
            for (i = 0; i < k && n != nullptr; i++) {
                if (i == 0) {
                    t = n;
                }
                auto next = n->next;
                n->next = h;
                h = n;
                n = next;
            }
            if (n == nullptr && i < k) {
                n = h;
                h = nullptr;
                while (n != nullptr) {
                    auto next = n->next;
                    n->next = h;
                    h = n;
                    n = next;
                }
            }
            if (g != nullptr) {
                g->next = h;
            } else {
                head = h;
            }
            g = t;
        }
        return head;
    }
};
