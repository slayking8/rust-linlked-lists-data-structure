use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct List {
    items: Option<Rc<RefCell<Node>>>,
    len: usize,
}

impl Node {
    fn new(data: i32) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Node {
            data: data,
            next: None,
        }));
    }
}

impl List {
    fn new(data: Option<i32>) -> Self {
        let Some(d) = data else {
            return List {
                items: None,
                len: 0,
            };
        };

        return List {
            items: Some(Node::new(d)),
            len: 1,
        };
    }

    fn print(&self) {
        let mut current = self.items.clone();
        while let Some(node) = current {
            println!("{}", node.borrow().data);
            current = node.borrow().next.clone();
        }
    }

    fn insert(&mut self, data: i32) {
        if self.items.is_none() {
            self.items = Some(Node::new(data));
            self.len += 1;
            return;
        }

        let mut current = self.items.clone();
        while let Some(node) = current {
            if node.borrow().next.is_none() {
                node.borrow_mut().next = Some(Node::new(data));
                self.len += 1;
                break;
            }
            current = node.borrow().next.clone();
        }
    }

    fn insert_at(&mut self, data: i32, idx: usize) {
        if idx > self.len {
            println!("the given index is out of range");
            return;
        }

        let mut index = 0;
        let mut current = self.items.clone();
        while let Some(node) = current {
            if index == idx {
                let new_node = Node::new(data);
                new_node.borrow_mut().next = node.borrow().next.clone();
                node.borrow_mut().next = Some(new_node);
                self.len += 1;
                break;
            }

            current = node.borrow().next.clone();
            index += 1;
        }
    }

    fn delete(&mut self) {
        if self.items.is_none() {
            return;
        }

        let mut previous: Option<Rc<RefCell<Node>>> = None;
        let mut current = self.items.clone();
        while let Some(node) = current.clone() {
            if node.borrow().next.is_none() {
                if let Some(prev) = previous.as_ref() {
                    prev.borrow_mut().next = None;
                    self.len -= 1;
                }
            }
            previous = current.clone();
            current = node.borrow().next.clone();
        }
    }

    fn delete_at(&mut self, index: usize) {
        if index > self.len {
            println!("the given index is out of range");
            return;
        }

        let mut previous: Option<Rc<RefCell<Node>>> = None;
        let mut current = self.items.clone();
        let mut idx = 0;
        while let Some(node) = current.clone() {
            if idx == index {
                if let Some(prev) = previous.as_ref() {
                    prev.borrow_mut().next = node.borrow().next.clone();
                    self.len -= 1;
                    break;
                }
            }
            previous = current.clone();
            current = node.borrow().next.clone();
            idx += 1;
        }
    }
}

fn main() {
    let mut list = List::new(None);
    list.insert(12);
    list.insert(13);
    list.insert(14);
    list.insert(15);
    list.insert(16);
    list.insert(17);

    list.print();
    println!(
        "[LEN]: {}\n-------------------------------------\n",
        list.len
    );
    // list.insert_at(33, 8);
    // list.delete();
    list.delete_at(9);
    list.print();
    println!(
        "[LEN]: {}\n-------------------------------------\n",
        list.len
    );
}
