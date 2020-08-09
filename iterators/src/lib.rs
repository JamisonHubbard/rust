// the below derive command is a compiler command
// it tells the compiler to implement the PartialEq
// and Debug traits for the Shoe struct
//
// Debug allows the struct to be printed in a standard
// format
//
// PartialEq allows instances of the Shoe struct to be
// compared for equality
//
// Eq is the trait for absolute equality, whereas PartialEq
// is for partial equality and is less rigorous
// Unless it's definitely needed, don't use Eq as it is more
// difficult and must be implemented by you, not the compiler

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn _shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 14,
                style: String::from("sandal")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            }
        ];

        let in_my_size = _shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }
}