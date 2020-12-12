class Solution {
public:
    vector<TreeNode*> find(TreeNode* root, TreeNode* node) {
        vector<TreeNode*> path;
        path.push_back(root);
        auto p = root;
        while (p != node) {
            if (p->left != nullptr) {
                p = p->left;
            } else if (p->right != nullptr) {
                p = p->right;
            } else {
                auto parent = p;
                do {
                    p = parent;
                    path.pop_back();
                    if (path.size() == 0) {
                        return path;
                    }
                    parent = path.back();
                } while (parent->right == p || parent->right == nullptr);
                p = parent->right;
            }
            path.push_back(p);
        }
        return path;
    }

    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        auto ppath = find(root, p);
        auto qpath = find(root, q);
        auto length = min(ppath.size(), qpath.size());
        TreeNode* lca = nullptr;
        for (size_t i = 0; i < length; i++) {
            if (ppath[i] == qpath[i]) {
                lca = ppath[i];
            }
        }
        return lca;
    }
};
