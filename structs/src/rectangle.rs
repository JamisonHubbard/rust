mod rectangle {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        /*fn area(&self) -> u32 {
            self.width * self.height
        }*/
    
        fn can_hold(&self, other: &Rectangle) -> bool {
            other.width < self.width && other.height < self.height
        }
    }
}