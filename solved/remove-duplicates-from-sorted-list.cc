class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        auto node = head;
        auto prev = head;
        while(node != nullptr) {
            if(node != prev) {
                if(prev->val == node->val) {
                    prev->next = node->next;
                } else {
                    prev = node;
                }
            }
            node = node->next;
        }
        return head;
    }
};
