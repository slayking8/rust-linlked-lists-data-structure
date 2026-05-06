# Linked list implementation

A linked list is a dynamic data structure made of nodes connected via pointers.
Each node contains data and a reference to the next node.
It allows efficient insertion and removal without shifting elements.

In Rust, we often use `Option<Rc<RefCell<Node>>>` to manage ownership and mutability safely.
`Rc` enables multiple ownership of nodes.
`RefCell` allows interior mutability at runtime.

Insertion typically involves traversing nodes until the last one is found.
Then the `next` pointer is updated to a new node.

Although flexible, linked lists are less cache-friendly than arrays.
They are useful when frequent insertions and deletions are needed.



