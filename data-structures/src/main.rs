use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct BinaryTree<T> {
    left: Option<Rc<RefCell<BinaryTree<T>>>>,
    right: Option<Rc<RefCell<BinaryTree<T>>>>,
    value: T
}

fn main() {
    let left = Rc::new(RefCell::new(BinaryTree::<i32>{
        left: None,
        right: None,
        value: 1337
    }));

    let right = Rc::new(RefCell::new(BinaryTree::<i32>{
        left: None,
        right: None,
        value: 1337
    }));

    let head = (BinaryTree {
        left: Some(Rc::clone(&left)),
        right: Some(Rc::clone(&right)),
        value: 0
    });

    println!("{:#?}", head);

    if let Some(node) = &head.right {
        node.borrow_mut().value = 3;
        println!("{:#?}", head);
    }


}
