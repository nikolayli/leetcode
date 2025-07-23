use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::utils::linked_list::ListNode;

#[derive(PartialEq, Eq)]
struct Wrapper(Box<ListNode>);

impl PartialOrd for Wrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Wrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.val.cmp(&other.0.val)
    }
}

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for node in lists.into_iter().flatten() {
            heap.push(Reverse(Wrapper(node)));
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;

        while let Some(Reverse(Wrapper(mut node))) = heap.pop() {
            let next = node.next.take();

            if let Some(next_node) = next {
                heap.push(Reverse(Wrapper(next_node)));
            }

            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! merge_k_lists_test {
        ($name:ident: $lists:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let lists: Vec<Option<Box<ListNode>>> = $lists
                    .into_iter()
                    .map(|arr| ListNode::from_slice(arr))
                    .collect();
                let expected = ListNode::from_slice($expected);
                assert_eq!(Solution::merge_k_lists(lists), expected);
            }
        };
    }

    merge_k_lists_test!(case1: vec![&[1, 4, 5], &[1, 3, 4], &[2, 6, 9]], &[1, 1, 2, 3, 4, 4, 5, 6, 9]);
    merge_k_lists_test!(case2: Vec::<&[i32]>::new(), &[]);
    merge_k_lists_test!(case3: vec![&[]], &[]);
}
