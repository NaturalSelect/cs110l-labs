use std::{fmt, clone};
use std::option::Option;

pub struct  LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    pub value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T:std::fmt::Display> fmt::Display for LinkedList<T> where  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}


impl<T:PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.head.eq(&other.head) && self.size == other.size
    }
}

impl<T:PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.next.eq(&other.next)
    }
}

// // 非递归实现
// impl<T:Clone> Clone for LinkedList<T> {
//     fn clone(&self) -> Self {
//         let mut new = LinkedList{head: None, size: self.size.clone()};
//         let mut current = &self.head;
//         let mut temp = vec![];
//         while let Some(ref1) = current {
//             temp.push(ref1);
//             current = &ref1.next;
//         }
//         let curr = temp.pop();
//         while let Some(content) = curr  {
//             new.push_front(content.value.clone())
//         }  
//         new
//     }
// }

impl<T:Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {value:self.value.clone(), next:self.next.clone()}
    }
}
impl<T:Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        LinkedList{head: self.head.clone(), size: self.size}
    }
}


