use crate::utils::linked_list::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n = n as usize;
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;

        {
            let mut curr = dummy.as_ref();
            while let Some(node) = curr {
                len += 1;
                curr = node.next.as_ref();
            }
        }

        let target_index = len - n - 1;

        let mut curr = dummy.as_mut();
        for _ in 0..target_index {
            curr = curr.and_then(|node| node.next.as_mut());
        }

        if let Some(node) = curr {
            let next_node = node.next.take();
            if let Some(next_node) = next_node {
                node.next = next_node.next;
            }
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list;

    macro_rules! remove_nth_from_end_test {
        ($name:ident: [$($head:expr),*], $n:expr => [$($expected:expr),*]) => {
            #[test]
            fn $name() {
                let head = list![$($head),*];
                let expected = list![$($expected),*];
                assert_eq!(Solution::remove_nth_from_end(head, $n), expected);
            }
        };
    }

    remove_nth_from_end_test!(case1: [1, 2, 3, 4, 5], 2 => [1, 2, 3, 5]);
    remove_nth_from_end_test!(case2: [1], 1 => []);
    remove_nth_from_end_test!(case3: [1, 2], 1 => [1]);
}
