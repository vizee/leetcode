class Solution {
public:
    bool isMirror(TreeNode* left, TreeNode* right) {
        if (left != nullptr && right != nullptr) {
            if (left->val != right->val) {
                return false;
            }
        } else {
            return (left == nullptr && right == nullptr);
        }
        return isMirror(left->left, right->right) && isMirror(left->right, right->left);
    }

    bool isSymmetric(TreeNode* root) {
        if (root == nullptr) {
            return true;
        }
        return isMirror(root->left, root->right);
    }
};
