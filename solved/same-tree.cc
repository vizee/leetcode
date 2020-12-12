class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q)
    {
        if (p == NULL || q == NULL) {
            return p == q;
        } else if (p->val != q->val) {
            return false;
        }
        return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
    }
};