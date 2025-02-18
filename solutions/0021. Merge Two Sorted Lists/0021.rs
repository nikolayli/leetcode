// Time complexity: O(∣list1∣+∣list2∣)
// Space complexity: O(1)
impl Solutions {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut list1 = list1.unwrap();
        let mut list2 = list2.unwrap();

        if list1.val > list2.val {
            std::mem::swap(&mut list1, &mut list2);
        }
        list1.next = Self::merge_two_lists(list1.next, Some(list2));

        Some(list1)
    }
}
