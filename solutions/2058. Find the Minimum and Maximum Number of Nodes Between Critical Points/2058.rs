// Time complexity: O(n)
// Space complexity: O(1)
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut min_distance = i32::MAX;
        let mut first_ma_index = -1;
        let mut prev_ma_index = -1;
        let mut index = 1;
        let mut prev = &head;
        let mut curr = &head.as_ref().unwrap().next;

        while let Some(curr_node) = curr {
            if let Some(next_node) = &curr_node.next {
                if (curr_node.val > prev.as_ref().unwrap().val && curr_node.val > next_node.val)
                    || (curr_node.val < prev.as_ref().unwrap().val && curr_node.val < next_node.val)
                {
                    if first_ma_index == -1 {
                        first_ma_index = index;
                    }
                    if prev_ma_index != -1 {
                        min_distance = min_distance.min(index - prev_ma_index);
                    }
                    prev_ma_index = index;
                }
            }
            prev = curr;
            curr = &curr_node.next;
            index += 1;
        }

        if min_distance == i32::MAX {
            vec![-1, -1]
        } else {
            vec![min_distance, prev_ma_index - first_ma_index]
        }
    }
}
