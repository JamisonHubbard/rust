use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    let _l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x1 = 5;
    let x2 = 5;
    let y1 = &x1;
    let y2 = Box::new(x2);

    println!("{}  {}", y1, y2);
}

enum List {
    Cons(i32, Box<List>),
    Nil
}