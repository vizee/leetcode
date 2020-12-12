impl Solution {
    fn remove_nth_from_end_impl(head: Option<Box<ListNode>>, n: i32, d: &mut i32) -> Option<Box<ListNode>> {
        head.and_then(|mut x| {
            let next = Self::remove_nth_from_end_impl(x.next, n, d);
            *d += 1;
            if *d == n {
                next
            } else {
                x.next = next;
                Some(x)
            }
        })
    }
    
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end_impl(head, n, &mut 0)
    }
}
