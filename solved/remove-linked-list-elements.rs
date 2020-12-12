impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        head.and_then(|mut x| if x.val == val {
            Solution::remove_elements(x.next, val)
        } else {
            x.next = Solution::remove_elements(x.next, val);
            Some(x)
        })
    }
}
