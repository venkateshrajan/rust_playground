use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct DoublyLinkedNode<T> 
    where T: Copy + Sized 
{
    data: T,
    next: Rc<Option<RefCell<Self>>>,
    prev: Rc<Option<RefCell<Self>>>,
}

struct DoublyLinkedList<T>
    where T: Copy + Sized
{
    head: Rc<RefCell<DoublyLinkedNode<T>>>,
    tail: Rc<RefCell<DoublyLinkedNode<T>>>,
}


impl<T> Clone for DoublyLinkedNode<T> 
    where T: Debug + Copy + Sized {
    fn clone(&self) -> Self {
        Self {
            data: self.data,
            next: Rc::clone(&self.next),
            prev: Rc::clone(&self.prev),
        }
    }
}

impl<T> DoublyLinkedNode<T> 
    where T: Copy + Debug + Sized 
{
    #[allow(dead_code)]
    fn new(data: T) -> Self {
        Self {
            data,
            next: None.into(),
            prev: None.into(),
        }
    }

    #[allow(dead_code)]
    fn create(data: T, next: Rc<Option<RefCell<Self>>>, prev: Rc<Option<RefCell<Self>>>) -> Self {
        Self {
            data,
            next,
            prev,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("{:?}", self.data);
        if let Some(next) = self.next.as_ref() {
            next.borrow().print();
        }
    }

    #[allow(dead_code)]
    fn print_reverse(&self) {
        println!("{:?}", self.data);
        if let Some(prev) = self.prev.as_ref() {
            prev.borrow().print_reverse();
        }
    }

    #[allow(dead_code)]
    fn add_next(&mut self, node: RefCell<Self>) {
        if !self.next.as_ref().is_some() {
            self.next = Some(node).into();
            if let Some(next) = self.next.as_ref() {
                next.borrow_mut().prev = Some(RefCell::new(self.clone())).into();
            }
        } 
    }

    #[allow(dead_code)]
    fn add_prev(&mut self, node: RefCell<Self>) {
        if !self.prev.as_ref().is_some() {
            self.prev = Some(node).into();
            if let Some(prev) = self.prev.as_ref() {
                prev.borrow_mut().next = Some(RefCell::new(self.clone())).into();
            }
        } 
    }
}

impl<T> DoublyLinkedList<T>
    where T: Copy + Debug + Sized 
{
    #[allow(dead_code)]
    fn new(node: Rc<RefCell<DoublyLinkedNode<T>>>) -> Self {
        Self {
            head: node.clone(),
            tail: node.clone(),
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.head.as_ref().borrow().print();
    }

    #[allow(dead_code)]
    fn print_reverse(&self) {
        self.tail.as_ref().borrow().print_reverse();
    }

    #[allow(dead_code)]
    fn push_back(&mut self, node: RefCell<DoublyLinkedNode<T>>) {
        self.tail.as_ref().borrow_mut().add_next(node.clone());
        self.tail = node.into();
    }

    #[allow(dead_code)]
    fn push_front(&mut self, node: RefCell<DoublyLinkedNode<T>>) {
        self.head.as_ref().borrow_mut().add_prev(node.clone());
        self.head = node.into();
    }
}

#[test]
fn it_works() {
    let mut list = DoublyLinkedList::new(RefCell::new(DoublyLinkedNode::new(3)).into());
    let node1 = RefCell::new(DoublyLinkedNode::new(0));
    let node2 = RefCell::new(DoublyLinkedNode::new(1));
    list.push_back(node1);
    list.push_back(node2);
    list.print();
    list.print_reverse();
}


