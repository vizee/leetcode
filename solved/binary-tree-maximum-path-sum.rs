impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_path_sum_subtree(root: &Rc<RefCell<TreeNode>>) -> i32 {
            let mut root = root.borrow_mut();
            let mut max = root.val;
            let mut leftval = 0;
            let mut rightval = 0;
            if let Some(ref p) = root.left {
                max = max.max(max_path_sum_subtree(p));
                leftval = p.borrow().val;
            }
            if let Some(ref p) = root.right {
                max = max.max(max_path_sum_subtree(p));
                rightval = p.borrow().val;
            }
            let max = max.max(root.val + leftval + rightval);
            root.val = root.val
                .max(root.val + leftval)
                .max(root.val + rightval);
            max.max(root.val)
        }

        if let Some(root) = root {
            max_path_sum_subtree(&root)
        } else {
            0
        }
    }
}
