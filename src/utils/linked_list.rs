#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
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

    pub fn from_slice(s: &[i32]) -> Option<Box<Self>> {
        let mut head = None;
        for &val in s.iter().rev() {
            let mut node = Self::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    pub fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut curr = head.as_ref();
        while let Some(node) = curr {
            vec.push(node.val);
            curr = node.next.as_ref();
        }
        vec
    }
}

#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($val:expr $(, $rest:expr)*) => {
        Some(Box::new(ListNode {
            val: $val,
            next: list!($($rest),*)
        }))
    };
}
