// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_list_node<T>(list: Vec<T>) -> Option<Box<ListNode<T>>> {
    let mut head = None;
    let mut current = &mut head;

    for item in list {
        *current = Some(Box::new(ListNode::new(item)));
        current = &mut current.as_mut().unwrap().next;
    }

    head
}
