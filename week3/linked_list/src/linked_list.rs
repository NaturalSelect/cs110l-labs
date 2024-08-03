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
        // 清理链表中的所有节点
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
            // 这里不需要做任何额外的工作，`node` 和 `current` 会自动释放
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
// 普通for循环获取ownership,所以迭代器ctx存的对象就是这个node
pub struct LinkedListIterator<T> {
    curr: Option<Box<Node<T>>>
}
// for in &collection, 不获取控制权，所以ctx存引用
pub struct RefLinkedListIterator<'a, T> {
    curr: &'a Option<Box<Node<T>>>
}

// into_for,将it;
// 这里需要head.take(),将链表表头节点option置为none; 并实际将头指针搬到迭代器ctx中
// 如果直接移动赋值,（相当于获取了struct部分控制权,导致在drop时self对象不完整了)
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;

    type IntoIter = LinkedListIterator<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        return LinkedListIterator{curr:self.head.take()};
    }
}

// 引用for,引用头节点即可
impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;

    type IntoIter = RefLinkedListIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return RefLinkedListIterator{curr:&self.head};
    }
}

// into_for的实际迭代器,返回option<T>
// 返回时需要从迭代器的option中take出来（否则没法拿出来这个唯一指针）
impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let cur = self.curr.take();
        match cur{
            Some(node) => {
                self.curr = node.next;
                Some(node.value)
            },
            None => None
        }
    }
}

// 引用 for的实际迭代器,返回option<&T>
// 返回时需要从迭代器的option中take出来（否则没法拿出来这个唯一指针）
impl<'a, T> Iterator for RefLinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        match self.curr{
            Some(node) => {
                self.curr = &node.next;
                Some(&node.value)
            },
            None => None
        }
    }
}

// // 非递归实现
// impl<T:Clone> Clone for LinkedList<T> {
//     fn clone(&self) -> Self {
//         let mut new = LinkedList{head: None, size: self.size.clone()};
//         let mut current = &self.head;
        // let mut temp = vec![];
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

