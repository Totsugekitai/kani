use alloc::sync::Arc;
use spin::Mutex;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Arc<Mutex<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
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

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.lock().prev.take() {
                Some(new_tail) => {
                    new_tail.lock().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Arc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn remove(&mut self, index: usize) {
        let mut remove_node = self.head.take();
        for _ in 0..index {
            if let Some(node) = remove_node {
                remove_node = node.lock().next.clone();
            } else {
                break;
            }
        }
        if let Some(node) = remove_node {
            let prev = node.lock().prev.clone();
            if let Some(prev) = prev {
                prev.lock().next = node.lock().next.clone();
            }
            let next = node.lock().next.clone();
            if let Some(next) = next {
                next.lock().prev = node.lock().prev.clone();
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}
