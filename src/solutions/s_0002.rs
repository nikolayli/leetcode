use crate::utils::linked_list::ListNode;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        let mut carry = 0;
        let mut p = l1;
        let mut q = l2;

        while p.is_some() || q.is_some() || carry != 0 {
            let sum = carry + p.as_ref().map_or(0, |node| node.val) + q.as_ref().map_or(0, |node| node.val);

            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();

            p = p.and_then(|node| node.next);
            q = q.and_then(|node| node.next);
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::ListNode;

    macro_rules! add_two_numbers_test {
        ($name:ident: ($l1:expr, $l2:expr) => $expected:expr) => {
            #[test]
            fn $name() {
                let l1 = ListNode::from_vec($l1.to_vec());
                let l2 = ListNode::from_vec($l2.to_vec());
                let expected = ListNode::from_vec($expected.to_vec());
                assert_eq!(Solution::add_two_numbers(l1, l2), expected);
            }
        };
    }

    add_two_numbers_test!(case1: ([2, 4, 3], [5, 6, 4]) => [7, 0, 8]);
    add_two_numbers_test!(case2: ([0], [0]) => [0]);
    add_two_numbers_test!(case3: ([9, 9, 9, 9, 9, 9, 9], [9, 9, 9, 9]) => [8, 9, 9, 9, 0, 0, 0, 1]);
}

