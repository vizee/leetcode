struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2)
{
    struct ListNode *h = NULL;
    struct ListNode *p;
    int v = 0;

    while (l1 || l2 || v) {
        if (h == NULL) {
            h = p = malloc(sizeof(struct ListNode));
        } else {
            p->next = malloc(sizeof(struct ListNode));
            p = p->next;
        }
        p->next = NULL;
        if (l1) {
            v += l1->val;
            l1 = l1->next;
        }
        if (l2) {
            v += l2->val;
            l2 = l2->next;
        }
        p->val = v % 10;
        v /= 10;
    }
    return h;
}
