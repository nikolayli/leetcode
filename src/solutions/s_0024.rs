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

    fn to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut curr = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = curr;
            curr = Some(Box::new(node));
        }
        curr
    }

    fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut curr = &head;
        while let Some(node) = curr {
            ans.push(node.val);
            curr = &node.next;
        }
        ans
    }

    macro_rules! swap_pairs_test {
        ($name:ident: $heat:expr => $expected:expr) => {
            #[test]
            fn $name() {
                let heat = to_list($heat);
                let ans = Solution::swap_pairs(heat);
                assert_eq!(to_vec(ans), $expected);
            }
        };
    }

    swap_pairs_test!(case1: &[1, 2, 3, 4] => vec![2, 1, 4, 3]);
    swap_pairs_test!(case2: &[] => vec![]);
    swap_pairs_test!(case3: &[1] => vec![1]);
    swap_pairs_test!(case4: &[1, 2, 3] => vec![2, 1, 3]);
}
