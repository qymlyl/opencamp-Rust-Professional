use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None,
            next: None,
        }
    }
}

struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    // 添加新节点
    pub fn add(&mut self, obj: T) {
        // 创建新节点
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;

        // 获取节点指针
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        // 将节点接入链表
        match self.end {
            // 第一个节点
            None => self.start = node_ptr,

            // 非第一个节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }

        self.end = node_ptr;
        self.length += 1;
    }

    // 根据索引获取节点的值
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(node_ptr) => match index {
                0 => Some(unsafe { &(*node_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*node_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 链表反转
    pub fn reverse(&mut self) {
        // 没有节点或者只有一个节点直接返回
        if self.start.is_none() || self.length == 1 {
            return;
        }

        let (mut prev, mut cur) = (None, self.start);

        //   NULL 1-->2-->3-->4-->5
        // prev  cur
        // next= --> cur.next-->3
        // cur.next = prev = NULL
        // cur.prev = next = 3
        // prev = cur = 1
        // cur = next = 2
        // NULL <--next--1 <--next-- 2 ---prev---> 3 -next-> 4 --next--> 5
        //               prev       cur
        while let Some(cur_ptr) = cur {
            let next = unsafe { (*cur_ptr.as_ptr()).next };
            unsafe { (*cur_ptr.as_ptr()).next = prev }
            unsafe { (*cur_ptr.as_ptr()).prev = next }

            // 指针后移
            prev = cur;
            cur = next;
        }

        self.end = self.start;
        self.start = prev;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_3() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![];
        let reverse_vec: Vec<i32> = vec![];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_4() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![1];
        let reverse_vec: Vec<i32> = vec![1];
        for i in 0..original_vec.len() {
            list.add(original_vec[i]);
        }
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }
}
