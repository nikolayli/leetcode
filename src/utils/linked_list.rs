#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    pub fn from_vec(v: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        for &val in v.iter().rev() {
            let mut node = Self::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

