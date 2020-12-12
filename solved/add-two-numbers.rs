impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut r = None;
        let mut cur = &mut r;
        let mut val = 0;
        while l1.is_some() || l2.is_some() ||  val > 0 {
            if let Some(n) = l1 {
                val += n.val;
                l1 = n.next;
            }
            if let Some(n) = l2 {
                val += n.val;
                l2 = n.next;
            }
            *cur = Some(Box::new(ListNode::new(val % 10)));
            cur = &mut cur.as_mut().unwrap().next;
            val /= 10;
        }
        r
    }
}
