use std::{cell::RefCell, collections::HashMap, rc::Rc};

// head存放None或者从head到tail依次存放最近到最早的数据
struct LRUCache {
    head: Link,
    tail: Link,
    // cached_map中Link一定是Some(_)
    cached_map: HashMap<i32, Link>,
    capacity: i32,
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            head: None,
            tail: None,
            cached_map: HashMap::new(),
            capacity,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.cached_map.contains_key(&key) {
            return -1;
        }
        if let Some(node) = self.cached_map.get(&key).unwrap() {
            let node = Rc::clone(node);
            let value = node.borrow().value;
            self.delete(&node);
            self.push_front(node);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cached_map.contains_key(&key) {
            let node = Rc::clone(self.cached_map.get(&key).unwrap().as_ref().unwrap());
            self.delete(&node);
        } else {
            if self.cached_map.len() >= self.capacity as usize {
                let node = Rc::clone(self.tail.as_ref().unwrap());
                self.delete(&node);
            }
        }
        let node = Node::new(key, value);
        self.push_front(Rc::clone(&node));
        self.cached_map.insert(key, Some(node));
    }

    fn push_front(&mut self, node: Rc<RefCell<Node>>) {
        match self.head.take() {
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(old_head);
                self.head = Some(Rc::clone(&node));
            }
        }
        let key = node.borrow().key;
        self.cached_map.insert(key, Some(node));
    }

    fn delete(&mut self, node: &Rc<RefCell<Node>>) {
        match (
            Rc::clone(node).borrow().prev.as_ref(),
            Rc::clone(node).borrow().next.as_ref(),
        ) {
            (None, None) => {
                self.head = None;
                self.tail = None;
            }
            (None, Some(next)) => {
                self.head = Some(Rc::clone(next));
                next.borrow_mut().prev = None;
            }
            (Some(prev), None) => {
                self.tail = Some(Rc::clone(prev));
                prev.borrow_mut().next = None;
            }
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev = Some(Rc::clone(prev));
                prev.borrow_mut().next = Some(Rc::clone(next));
            }
        }
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;
        self.cached_map.remove(&node.borrow().key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_case() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
