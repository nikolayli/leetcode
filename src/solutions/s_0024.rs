use crate::utils::linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut prev = &mut dummy;

        while let Some(mut curr) = prev.next.take() {
            if let Some(mut next) = curr.next.take() {
                let next_next = next.next.take();
                curr.next = next_next;
                next.next = Some(curr);
                prev.next = Some(next);
                prev = prev.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                prev.next = Some(curr);
                break;
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::ListNode;

    macro_rules! swap_pairs_test {
        ($name:ident: $head:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let head = ListNode::from_slice($head);
                let expected = ListNode::from_slice($expected);
                assert_eq!(Solution::swap_pairs(head), expected);
            }
        };
    }

    swap_pairs_test!(case1: &[1, 2, 3, 4] => &[2, 1, 4, 3]);
    swap_pairs_test!(case2: &[] => &[]);
    swap_pairs_test!(case3: &[1] => &[1]);
    swap_pairs_test!(case4: &[1, 2, 3] => &[2, 1, 3]);
}
