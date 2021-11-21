use alloc::sync::Arc;
use spin::Mutex;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    pub tail: Link<T>,
}

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.lock().next = Some(new_tail.clone());
                new_tail.lock().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.lock().next.take() {
                Some(new_head) => {
                    new_head.lock().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Arc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
