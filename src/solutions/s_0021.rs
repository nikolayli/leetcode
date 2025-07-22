use crate::utils::linked_list::ListNode;

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            if l1.val <= l2.val {
                let mut node = list1.take().unwrap();
                list1 = node.next.take();
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            } else {
                let mut node = list2.take().unwrap();
                list2 = node.next.take();
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
        }
        tail.next = list1.or(list2);
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut curr = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = curr;
            curr = Some(Box::new(node));
        }
        curr
    }

    macro_rules! merge_two_lists_test {
        ($name:ident: ($list1:expr, $list2:expr) => $expected:expr) => {
            #[test]
            fn $name() {
                let list1 = to_list($list1);
                let list2 = to_list($list2);
                let expected = to_list($expected);
                assert_eq!(Solution::merge_two_lists(list1, list2), expected);
            }
        };
    }

    // merge_two_lists_test!(case1: (vec![1, 2, 4], vec![1, 3, 4]) => vec![1, 1, 2, 3, 4, 4]);
    merge_two_lists_test!(case2: (vec![], vec![]) => vec![]);
    merge_two_lists_test!(case3: (vec![], vec![0]) => vec![0]);
}
