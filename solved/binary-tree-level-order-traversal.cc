class Solution {
public:
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> vv;
        list<TreeNode*> q;
        q.push_back(root);
        q.push_back(nullptr);
        vector<int> v;
        while(!q.empty()) {
            auto f = q.front();
            q.pop_front();
            if(f == nullptr) {
                if(v.size() != 0) {
                    vv.push_back(v);
                } else {
                    break;
                }
                q.push_back(nullptr);
                v = vector<int>();
            } else {
                v.push_back(f->val);
                if(f->left != nullptr) {
                    q.push_back(f->left);
                }
                if(f->right != nullptr) {
                    q.push_back(f->right);
                }
            }
        }
        return vv;
    }
};
