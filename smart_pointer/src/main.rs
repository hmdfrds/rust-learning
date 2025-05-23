use std::{cell::RefCell, rc::Rc};

struct Gadget {
    id: u32,
    data: RefCell<String>,
}

fn main() {
    let gadget = Rc::new(Gadget {
        id: 1,
        data: RefCell::new("Initial Gadget Data".to_string()),
    });
    let owner_a = Rc::clone(&gadget);
    let owner_b = Rc::clone(&gadget);

    println!(
        "Data from owner a: {}. Current length: {}.",
        owner_a.data.borrow(),
        owner_a.data.borrow().len()
    );

    owner_b.data.borrow_mut().push_str("- updated!");

    println!(
        "Data from owner a: {}. Current length: {}",
        gadget.data.borrow(),
        gadget.data.borrow().len()
    );
}
