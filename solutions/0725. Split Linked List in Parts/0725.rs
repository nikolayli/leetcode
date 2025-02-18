// Time complexity: O(n)
// Space complexity: O(k)
impl Solution {
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let length = Self::get_length(&root);
        let sub_length = length / k;
        let mut remainder = length % k;

        let mut ans = vec![None; k as usize];
        let mut head = root;
        let mut prev: Option<Box<ListNode>> = None;

        for i in 0..k {
            let mut curr_length = sub_length + if remainder > 0 { 1 } else { 0 };
            remainder -= 1;

            let mut new_head = None;
            let mut tail = &mut new_head;

            while curr_length > 0 && head.is_some() {
                curr_length -= 1;
                let mut node = head.take().unwrap();
                head = node.next.take();

                tail = &mut tail.insert(node).next;
            }

            if let Some(node) = new_head {
                ans[i as usize] = Some(node);
            }
        }

        ans
    }

    fn get_length(root: &Option<Box<ListNode>>) -> i32 {
        let mut length = 0;
        let mut node = root;

        while let Some(n) = node {
            length += 1;
            node = &n.next;
        }

        length
    }
}
