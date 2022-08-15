use std::mem;

type ListLink<T: PartialOrd> = Option<Box<ListNode<T>>>;
pub struct LL<T: PartialOrd> {
    len: usize,
    head: ListLink<T>,
}

struct ListNode<T: PartialOrd> {
    next: ListLink<T>,
    value: T,
}

impl<T: PartialOrd> LL<T> {
    pub fn new() -> Self {
        return Self { len: 0, head: None };
    }

    pub fn push(&mut self, value: T) {
        let node = ListNode {
            next: mem::replace(&mut self.head, None),
            value,
        };
        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            let front = mem::replace(&mut self.head, None).unwrap();
            self.head = front.next;
            self.len -= 1;
            return Some(front.value);
        }
        return None;
    }

    pub fn contains(&self, value: T) -> bool {
        let mut curr: &ListLink<T> = &self.head;
        loop {
            if let Some(link) = curr {
                if link.value == value {
                    return true;
                }
                curr = &link.next;
            } else {
                return false;
            }
        }
    }

    pub fn len(&mut self) -> usize {
        return self.len;
    }
}

#[cfg(test)]
mod ll_tests {
    use crate::linked_list::LL;

    #[test]
    fn new() {
        let mut ll: LL<i32> = LL::new();
        assert_eq!(ll.len(), 0);
    }

    #[test]
    fn push_contains() {
        let mut ll: LL<i32> = LL::new();
        let val: i32 = 123;
        ll.push(val);
        assert!(ll.contains(val));
        assert_eq!(ll.len(), 1);
    }

    #[test]
    fn push_string() {
        let mut ll: LL<String> = LL::new();
        let val: String = String::from("Testing");
        ll.push(val.to_string());
        assert!(ll.contains(val));
    }

    #[test]
    fn pop_empty() {
        let mut ll: LL<i32> = LL::new();
        assert!(ll.pop().is_none());
    }

    #[test]
    fn pop() {
        let mut ll: LL<i32> = LL::new();
        let val: i32 = 456;
        ll.push(val);
        assert_eq!(ll.pop().unwrap(), val);
    }
}
