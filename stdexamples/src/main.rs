use std::cell::RefCell;
use std::rc::Rc;

struct LinkedNode {
    data: u32,
    next: Rc<Option<RefCell<Self>>>
}

impl LinkedNode {
    fn new(data:u32) -> Self {
        Self {
            data,
            next: None.into(),
        }
    }

    fn create(data: u32, next: Rc<Option<RefCell<Self>>>) -> Self {
        Self {
            data,
            next,
        }
    }

    fn print(&self) {
        println!("{}", self.data);
        if let Some(next) = self.next.as_ref() {
            next.borrow().print();
        }
    }

    fn push_back(&mut self, node: RefCell<Self>) {
        if let Some(next) = self.next.as_ref() {
            next.borrow_mut().push_back(node);
        } else {
            self.next = Some(node).into();
        }
    }

    fn push_front(&mut self, node: RefCell<Self>) {
        let another_node = Self::create(self.data, Rc::clone(&self.next));
        self.data = node.borrow().data;
        node.borrow_mut().data = another_node.data;
        node.borrow_mut().next = another_node.next;
        self.next = Some(node).into(); 
    }
}

fn main() {
    let mut root = LinkedNode::new(0);
    let node1 = LinkedNode::new(1);
    let node2 = LinkedNode::new(2);
    root.push_back(RefCell::new(node1));
    root.push_front(RefCell::new(node2));

    root.print();
}
