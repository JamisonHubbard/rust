use crate::rectangle::Rectangle;

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 60,
    };

    println!(
        "Can rect1 contain the other two?\n\trect2: {}\n\trect3: {}",
        rect1.can_hold(&rect2),
        rect1.can_hold(&rect3)
    );
    println!(
        "Can rect2 contain the other two?\n\trect1: {}\n\trect3: {}",
        rect2.can_hold(&rect1),
        rect2.can_hold(&rect3)
    );
    println!(
        "Can rect3 contain the other two?\n\trect1: {}\n\trect2: {}",
        rect3.can_hold(&rect1),
        rect3.can_hold(&rect2)
    );
}

mod rectangle {
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }
    
    impl Rectangle {
        /*fn area(&self) -> u32 {
            self.width * self.height
        }*/
    
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            other.width < self.width && other.height < self.height
        }
    }
}