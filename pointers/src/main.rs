use crate::BoxList::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    box_cons_list();


}

enum BoxList {
    Cons(i32, Box<BoxList>),
    Nil
}

fn box_cons_list() {
    let _l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x1 = 5;
    let x2 = 5;

    let y1 = &x1;
    let y2 = Box::new(x2);

    println!("{}  {}", y1, y2);
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil
}

fn rc_cons_list() {
    let shared_list = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));

    let a = Rc::new(RcList::Cons(0, Rc::clone(&shared_list)));
    let b = Rc::new(RcList::Cons(1, Rc::clone(&shared_list)));

    // ownership of shared_list is shared by both a and b
    // and Rust won't clean up shared_list until both a and
    // b are cleaned up or no longer own shared_list
}