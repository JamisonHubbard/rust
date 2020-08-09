

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: Comparable<T> + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item.gt(&largest) {
            largest = item;
        }
    }

    largest
}

trait Comparable<T> {
    fn gt(&self, other: &T) -> bool;
    fn eq(&self, other: &T) -> bool;
    fn lt(&self, other: &T) -> bool;
}

impl<T: PartialOrd> Comparable<T> for T {
    fn gt(&self, other: &T) -> bool {
        self > other
    }
    fn eq(&self, other: &T) -> bool {
        self == other
    }
    fn lt(&self, other: &T) -> bool {
        self < other
    }
}