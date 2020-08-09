#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn _can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn _get_width(&self) -> u32 {
        self.width
    }

    pub fn _get_height(&self) -> u32 {
        self.height
    }

    pub fn _swallow(&mut self, other: Rectangle) {
        self.height += other._get_height();
        self.width += other._get_width();
    }
}

pub fn _get_larger() -> Rectangle {
    Rectangle {
        width: 8,
        height: 7
    }
}

pub fn _get_smaller() -> Rectangle {
    Rectangle {
        width: 5,
        height: 1
    }
}


// the below metadata "#[cfg(test)]" tells Rust to only compile this
// module when running "cargo test"
#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let _larger = _get_larger();
        let _smaller = _get_smaller();
        assert!(_larger._can_hold(&_smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let _larger = _get_larger();
        let _smaller = _get_smaller();
        assert!(_larger._can_hold(&_smaller));
    }

    #[test]
    fn larger_correct_height() {
        let _larger = _get_larger();
        let _smaller = _get_smaller();
        assert_eq!(7, _larger._get_height());
    }

    #[test]
    #[ignore]
    fn smaller_nonzero_width() {
        let _larger = _get_larger();
        let _smaller = _get_smaller();
        assert_ne!(0, _smaller._get_width());
    }

    #[test]
    #[should_panic]
    fn incorrect_indexing() {
        let v = vec![0; 5];
        println!("Impossible value: {}", v[6]);
    }
}