// floyd's tortoise and hare
impl Solution {
    ///
    /// It's too difficult to write a O(1) space code in RUST (the borrow check...)  
    ///
    /// - runtime complexity: O(n)  
    /// - memory complexity: O(n)  
    ///
    /// the old version use O(n) space, consider modifying the list in-place using pointer manipulation instead of creating a new list to achieve O(1) space.
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let dummy = Some(Box::new(ListNode {
            val: -999,
            next: head,
        }));

        let mut hare = dummy.as_ref();
        for _ in 0..(n) {
            hare = hare.and_then(|t| t.next.as_ref());
        }
        let mut new_dummy = Some(Box::new(ListNode {
            val: -999,
            next: None,
        }));
        let mut new_tail = &mut new_dummy;
        let mut tortoise = dummy.as_ref();
        while hare.is_some() {
            //move forward together
            hare = hare.expect("impossible").next.as_ref();
            tortoise = tortoise.expect("impossible").next.as_ref();
            new_tail = new_tail.as_mut().map(|t| &mut t.next).unwrap();
            //copy tortoise node to new chain
            *new_tail = tortoise.as_ref().map(|t| Box::new(ListNode::new(t.val)));
        }
        tortoise.map(|t| *new_tail = t.next.clone());
        new_dummy.expect("impossible").next
    }
}

// below are default definition of this problem
struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn build_chain(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut new_head: Option<Box<Self>> = None;
        let mut ptr = &mut new_head;
        for ele in vec {
            if ptr.is_none() {
                ptr.replace(Box::new(ListNode::new(ele)));
            } else {
                ptr.as_mut().map(|t| {
                    t.next.replace(Box::new(ListNode::new(ele)));
                });
                ptr = ptr.as_mut().map(|t| &mut t.next).unwrap();
            }
        }
        new_head
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::remove_nth_from_end(ListNode::build_chain(vec![1, 2, 3, 4, 5]), 2),
        ListNode::build_chain(vec![1, 2, 3, 5])
    );

    assert_eq!(
        Solution::remove_nth_from_end(Some(Box::new(ListNode::new(1))), 1),
        None
    );

    assert_eq!(
        Solution::remove_nth_from_end(ListNode::build_chain(vec![1, 2]), 1),
        Some(Box::new(ListNode::new(1)))
    );

    assert_eq!(
        Solution::remove_nth_from_end(ListNode::build_chain(vec![1, 2, 3]), 3),
        ListNode::build_chain(vec![2, 3])
    );
}
