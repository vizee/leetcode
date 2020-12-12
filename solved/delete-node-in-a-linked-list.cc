class Solution {
public:
    inline void deleteNode(ListNode* node) {
        if(node->next != nullptr) {
            *node = *node->next;
        }
    }

};
