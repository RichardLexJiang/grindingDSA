// floyd's tortoise and hare
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end_v1(head, n)
    }

    /// # V1 version
    ///
    /// - runtime complexity: O(n)  
    /// - memory complexity: O(n)  
    ///
    /// the old version use O(n) space, consider modifying the list in-place using pointer manipulation instead of creating a new list to achieve O(1) space.
    pub fn remove_nth_from_end_v1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let Some(head) = head else {
            return None;
        };
        let mut i = 1;
        let mut hare = &head;
        while i < n {
            hare = hare.next.as_ref().expect("impossible");
            i += 1;
        }

        let mut new_head: Option<Box<ListNode>> = None;
        let mut new_list_ptr = &mut new_head;
        let mut tortoise = Some(&head);
        let mut removed_flag = false;
        while tortoise.is_some() {
            if removed_flag || hare.next.is_some() {
                let current_value = tortoise.as_ref().map(|t| t.val).unwrap();
                // set the value from tortoise
                if new_list_ptr.is_none() {
                    new_list_ptr.replace(Box::new(ListNode::new(current_value)));
                } else {
                    if let Some(ptr) = new_list_ptr.as_mut() {
                        ptr.next.replace(Box::new(ListNode::new(current_value)));
                    }
                    new_list_ptr = new_list_ptr.as_mut().map(|t| &mut t.next).unwrap();
                }
            } else {
                removed_flag = true;
            }

            tortoise = tortoise.and_then(|t| t.next.as_ref());
            if !removed_flag {
                hare = hare.next.as_ref().unwrap();
            }
        }

        new_head
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
